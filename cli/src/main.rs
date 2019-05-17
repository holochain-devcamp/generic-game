use std::io;
use std::iter::repeat;
use std::time::{self, SystemTime, UNIX_EPOCH};
use std::thread;
use serde_json::json;
use structopt::StructOpt;
use linefeed::{Interface, ReadResult};

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Cli {
	/// Url to connect to the running conductor HTTP port (e.g. http://localhost:3000)
	url: reqwest::Url,
	/// This is the instance ID in the conductor that is running the game on the given port (e.g gameInstance)
	instance: String,
}

static COMMANDS: &[(&str, &str)] = &[
    ("help",             "Displays this the help page"),
    ("join_game",        "Set the game to make moves against, usage: join_game <game_address>"),
    ("new_game",         "Create a new game to play with an opponent, usage: new_game <opponent_address>"),
    ("moves",            "Display the set of moves this game supports"),
    ("make_move",        "Make a move in this game, usage: make_move <move_json>"),
    ("exit",             "Exit this CLI. Holochain will persist state so games can be resumed later."),
];

fn main() -> io::Result<()> {
    let cli = Cli::from_args();

    // create the functions required for playing the game
    let whoami = holochain_call_generator(cli.url.clone(), cli.instance.clone(), "main".into(), "whoami".into());
    let valid_moves = holochain_call_generator(cli.url.clone(), cli.instance.clone(), "main".into(), "valid_moves".into());
    let make_move = holochain_call_generator(cli.url.clone(), cli.instance.clone(), "main".into(), "make_move".into());
    let create_game = holochain_call_generator(cli.url.clone(), cli.instance.clone(), "main".into(), "create_game".into());
    let render_game = holochain_call_generator(cli.url.clone(), cli.instance.clone(), "main".into(), "render_state".into());

    let interface = Interface::new("Holochain generic game")?;

    println!("");
    println!("");
    println!("{}", repeat('#').take(70).collect::<String>());
    println!("CLI interface for games written using the Holochain Generic Game framework.");
    println!("Enter \"help\" for a list of commands.");
    println!("Use \"create_game <agent_id>\" or \"join_game <game_address>\" to start or join a game.");
    println!("Press Ctrl-D or enter \"quit\" to exit.");
    println!("{}", repeat('#').take(70).collect::<String>());
    println!("");
    println!("");

    match whoami(json!({})) {
    	Ok(agent_addr) => {
    		println!("Your agent address is {}\n\nSend this to other players so they can invite you to a game.", agent_addr);
    	},
    	Err(_e) => {
    		println!("No holochain instance named {} running on {}. Check the conductor is running and the instanceId in the conductor config is correct.", cli.instance, cli.url);
    		return Ok(());
    	}
    }

    println!("");
    println!("");

	interface.set_prompt("No game> ")?;

	let mut current_game: Option<String> = None;

 	while let ReadResult::Input(line) = interface.read_line()? {

        if !line.trim().is_empty() {
            interface.add_history_unique(line.clone());
		}

        let (cmd, args) = split_first_word(&line);

        let result: Result<(), String> = match cmd {
            "help" => {
                println!("Holochain generic game commands:");
                println!();
                for &(cmd, help) in COMMANDS {
                    println!("  {:15} - {}", cmd, help);
                }
                println!();
                Ok(())
			}
            "join_game" => {
            	if is_hash(args) {
            		println!("Setting current game hash to {}", args);
            		current_game = Some(args.into());
                    Ok(())
            	} else {
            		Err("argument must be a valid address".into())
            	}
            }
            "new_game" => {
            	if is_agent_addr(args) {
            		let result = create_game(json!({
            			"opponent": args,
            			"timestamp": current_timestamp()
            		}));
                    result.map(|result| {
                        current_game = result.as_str().map(|s| s.to_string());
                    })
            	} else {
            		Err("argument must be valid agent address of an opponent.".into())
            	}
            }
            "moves" => {
            	valid_moves(json!({})).map(|result| {
	            	println!("The valid moves are:");
	            	result.as_array().unwrap()
	            	.iter()
	            	.for_each(|elem| {
	            		println!("- {}", elem);
	            	});
                    println!();
            	})
            },
            "make_move" => {
            	if let Some(current_game) = current_game.clone() {
            		let move_json: serde_json::Value = serde_json::from_str(args).unwrap();
	            	println!("making move: {:?}", args);
	            	make_move(json!({
		            	"game_move": {
		            		"game": current_game,
		            		"move_type": move_json,
		            		"timestamp": current_timestamp()
		            	}
	            	})).map(|_| {
                        println!("Move cast successfully");
                        println!("Waiting for gossip...");
                        // wait a bit so it displays correctly
                        thread::sleep(time::Duration::from_millis(4000));
                        println!("OK!")
                    })
                }
            	else {
            		Err("No game set to make moves on. use the \"join_game\" command.".into())
            	}
            },
            "exit" => {
            	if let Some(current_game) = current_game.clone() {
					println!("You can resume this game at a later date by using:\n\"join_game {}\"", current_game);
            	}
            	println!("Bye!");
            	break
            }
            _ => {
            	Err("Invalid command!".into())
            }
		};

        if let Err(e) = result {
            println!("Error: {}", e)
        }

		if let Some(current_game_string) = current_game.clone() {
 			interface.set_prompt(&format!("{}> ", current_game_string))?;
 			match render_game(json!({"game_address": current_game_string.clone()})) {
 				Ok(render_result) => {
            		println!("{}", render_result.as_str().unwrap());
 				},
 				Err(_e) => {
 					println!("No game is currently visible with that address.");
                    current_game = None;
 				}
 			}
 		}
	}
    Ok(())
}


/**
 * Returns functions to make calls to a particular zome function on a url
 */
fn holochain_call_generator(
	url: reqwest::Url, 
	instance: String,
	zome: String,
	func: String,
) -> Box<Fn(serde_json::Value) -> Result<serde_json::Value, String>> {

	let client = reqwest::Client::new();

	let make_rpc_call = move |params| {
		json!({
			"jsonrpc": "2.0",
			"id": 0,
			"method": "call",
			"params": {
				"instance_id": instance,
				"zome": zome,
				"function": func,
				"params": params
			}
		})
	};

	Box::new(move |params| {
		let call_result: serde_json::Value = client.post(url.clone())
		    .json(&make_rpc_call(params))
		    .send().map_err(|e| e.to_string())?
		    .json()
		    .map(|r: serde_json::Value| {
		    	r["result"].clone()
		    })
		    .map(|s| serde_json::from_str(
                s.as_str().expect(&format!("Holochain did not return a string result: {}", s))
            ).expect(&format!("Holochain did not return a valid stringified JSON result: {}", s)))
		    .map_err(|e| e.to_string())?;

		// deal with the json encoded holochain error responses
		if let Some(inner_result) = call_result.get("Ok") {
			Ok(inner_result.clone())
		} else {
			Err(call_result["Err"].to_string())
		}
	})

}

/*===============================
=            Helpers            =
===============================*/

fn split_first_word(s: &str) -> (&str, &str) {
    let s = s.trim();

    match s.find(|ch: char| ch.is_whitespace()) {
        Some(pos) => (&s[..pos], s[pos..].trim_start()),
        None => (s, "")
    }
}

fn is_hash(s: &str) -> bool {
	s.starts_with("Qm") && s.len() == 46
}

fn is_agent_addr(s: &str) -> bool {
	s.starts_with("Hc") && s.len() == 63
}

fn current_timestamp() -> u32 {
	SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32
}

/*=====  End of Helpers  ======*/
