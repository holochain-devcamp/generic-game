const {results, lastResult, makeMove, createGame, renderState, getState} = require('./helpers')

module.exports = (scenario) => {
	scenario("Can create a new game of checkers and make a move", async (s, t, { alice, bob }) => {
      
      const whoamiResult = await alice.callSync("main", "whoami", {})
      console.log(whoamiResult)
      t.equal(whoamiResult.Ok.length, 63)

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
	  console.log(lastResult())
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


	  // finally print all the outputs
	  results.forEach((result, i) => {
	    console.log(`${i}: ${JSON.stringify(result, null, 2)}\n`)
	  })

	})
} 
