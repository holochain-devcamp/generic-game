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


// scenario.runTape("Can create a new game of checkers and make a move", async (t, { alice, bob }) => {

//   // helpers
//   let results = []
//   const lastResult = (back=0) => results[results.length-1-back]
//   const makeMove = async (agent, game_move) => {
//     const result = await agent.callSync("main", "make_move", { game_move })
//     results.push(result)
//     return result
//   }
//   const getState = async (agent, game_address) => {
//     const result = await agent.callSync("main", "render_state", { game_address })
//     results.push(result)
//     return result
//   }

//   let create_result = await alice.callSync("main", "create_game", { opponent: bob.agentId, timestamp: 0 })
//   console.log(create_result)
//   t.equal(create_result.Ok.length, 46)
//   let game_address = create_result.Ok

//   await makeMove(bob, {
//     game: game_address,
//     timestamp: 0,
//     move_type: {MovePiece: { from: {x: 1, y: 5}, to: {x: 0, y: 4} }},
//   })
//   t.notEqual(lastResult().Ok, undefined)

//   await getState(alice, game_address)
//   console.log(lastResult().Ok)

//   await makeMove(alice, {
//   	game: game_address,
//   	timestamp: 1,
//   	move_type: {MovePiece: { from: {x: 0, y: 2}, to: {x: 1, y: 3} }},
//   })
//   t.notEqual(lastResult().Ok, undefined)

//   await getState(alice, game_address)
//   console.log(lastResult().Ok)

//   // await makeMove(bob, {
//   //   game: game_address,
//   //   timestamp: 2,
//   //   move_type: {MovePiece: { from: {x: 0, y: 0}, to: {x: 0, y: 0} }},
//   // })
//   // t.notEqual(lastResult().Ok, undefined)

//   // await makeMove(alice, {
//   //   game: game_address,
//   //   timestamp: 2,
//   //   move_type: {MovePiece: { from: {x: 0, y: 0}, to: {x: 0, y: 0} }},
//   // })
//   // t.notEqual(lastResult().Ok, undefined)

//   // t.equal(lastResult().Ok.moves.length, 4)

//   // both agents should see the same game state
//   // t.deepEqual(await getState(bob, game_address), await getState(alice, game_address))


//   // // finally print all the outputs
//   // results.forEach((result, i) => {
//   //   console.log(`${i}: ${JSON.stringify(result, null, 2)}\n`)
//   // })

// })

scenario.runTape("Can create a new game of tic-tac-toe and make a move", async (t, { alice, bob }) => {

  // helpers
  let results = []
  const lastResult = (back=0) => results[results.length-1-back]
  const makeMove = async (agent, game_move) => {
    const result = await agent.callSync("main", "make_move", { game_move })
    results.push(result)
    const state = await agent.callSync("main", "render_state", { game_address })
    console.log()
    console.log(state.Ok)
    console.log()
    return result
  }
  const getState = async (agent, game_address) => {
    const result = await agent.callSync("main", "render_state", { game_address })
    results.push(result)
    return result
  }

  // can get the descripton of the valid moves
  let valid_moves_result = await alice.callSync("main", "valid_moves", {})
  console.log(JSON.stringify(valid_moves_result.Ok))
  t.equal(valid_moves_result.Ok.length, 1)


  let create_result = await alice.callSync("main", "create_game", { opponent: bob.agentId, timestamp: 0 })
  console.log(create_result)
  t.equal(create_result.Ok.length, 46)
  let game_address = create_result.Ok

  await makeMove(bob, {
    game: game_address,
    timestamp: 0,
    move_type: {Place: { pos: {x: 0, y: 0} }},
  })
  t.notEqual(lastResult().Ok, undefined)


  await makeMove(alice, {
    game: game_address,
    timestamp: 1,
    move_type: {Place: { pos: {x: 1, y: 1} }},
  })
  t.notEqual(lastResult().Ok, undefined)


  await makeMove(bob, {
    game: game_address,
    timestamp: 2,
    move_type: {Place: { pos: {x: 0, y: 2} }},
  })
  t.notEqual(lastResult().Ok, undefined)


  await makeMove(alice, {
    game: game_address,
    timestamp: 2,
    move_type: {Place: { pos: {x: 2, y: 0} }},
  })
  t.notEqual(lastResult().Ok, undefined)


  await makeMove(bob, {
    game: game_address,
    timestamp: 2,
    move_type: {Place: { pos: {x: 0, y: 1} }},
  })
  t.notEqual(lastResult().Ok, undefined)


  // finally print all the outputs
  results.forEach((result, i) => {
    console.log(`${i}: ${JSON.stringify(result, null, 2)}\n`)
  })

})
