// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const { Config, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require("tape"))

const dnaPath = "./dist/generic-game.dna.json"
const agentAlice = Config.agent("alice")
const agentBob = Config.agent("bob")

const dna = Config.dna(dnaPath)
const instanceAlice = Config.instance(agentAlice, dna)
const instanceBob = Config.instance(agentBob, dna)
const scenario = new Scenario([instanceAlice, instanceBob], {debugLog: true})

scenario.runTape("Can create a new game and make a move", async (t, { alice, bob }) => {

  let create_result = await alice.callSync("main", "create_game", { opponent: bob.agentId, timestamp: 0 })
  console.log(create_result)
  t.equal(create_result.Ok.length, 46)
  let game_address = create_result.Ok

  let move_result = await alice.callSync("main", "make_move", { game_move: {
  	game: game_address,
  	move_type: {MovePiece: { from: {x: 0, y: 0}, to: {x: 0, y: 0} }},
  }})
  console.log(move_result)
  t.notEqual(move_result.Ok, undefined)

  let moves_result = await alice.callSync("main", "get_moves", { game_address })
  console.log(moves_result)
  t.notEqual(moves_result.Ok, undefined)
  t.equal(moves_result.Ok.length, 1)

})
