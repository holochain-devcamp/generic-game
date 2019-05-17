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
const scenario = new Scenario([instanceAlice, instanceBob], {debugLog: false})



// helpers
let results = []
const lastResult = (back=0) => results[results.length-1-back]
const makeMove = async (agent, game_move) => {
  const result = await agent.callSync("main", "make_move", { game_move })
  results.push(result)
  return result
}
const createGame = async (agent, opponent) => {
  const result = await agent.callSync("main", "create_game", { opponent: opponent.agentId, timestamp: 0 })
  results.push(result)
  return result.Ok
}
const renderState = async (agent, game_address) => {
  const result = await agent.callSync("main", "render_state", { game_address })
  console.log(result.Ok)
}
const getState = async (agent, game_address) => {
  const result = await agent.callSync("main", "get_state", { game_address })
  results.push(result)
  return result
}




scenario.runTape("Can create a new game of checkers and make a move", async (t, { alice, bob }) => {

  let game_address = await createGame(alice, bob);

  // agent 2 must go first
  await makeMove(bob, {
    game: game_address,
    timestamp: 0,
    move_type: {MovePiece: { from: {x: 1, y: 5}, to: {x: 0, y: 4} }},
  })
  t.notEqual(lastResult().Ok, undefined, "Bob made the first move")

  await renderState(alice, game_address)

  await makeMove(alice, {
  	game: game_address,
  	timestamp: 1,
  	move_type: {MovePiece: { from: {x: 0, y: 2}, to: {x: 1, y: 3} }},
  })
  t.notEqual(lastResult().Ok, undefined, "Alice made the second move")

  await renderState(alice, game_address)

  await makeMove(bob, {
    game: game_address,
    timestamp: 2,
    move_type: {MovePiece: { from: {x: 5, y: 5}, to: {x: 6, y: 4} }},
  })
  t.notEqual(lastResult().Ok, undefined, "Bob made the third move")

  let state = await getState(alice, game_address)

  t.equal(state.Ok.moves.length, 3, "There were three moves in the game")

  // both agents should see the same game state
  t.deepEqual(await getState(bob, game_address), await getState(alice, game_address), "Alice and Bob both see the same game state")


  // // finally print all the outputs
  // results.forEach((result, i) => {
  //   console.log(`${i}: ${JSON.stringify(result, null, 2)}\n`)
  // })

})
