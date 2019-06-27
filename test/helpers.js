
/**
 * Collection of functions to abstract the Holochain calls
 * and make the testing code cleaner.
 */

let results = []

module.exports = {
  results: results,
  lastResult: (back=0) => results[results.length-1-back],
  makeMove: async (agent, game_move) => {
    const result = await agent.callSync("main", "make_move", { new_move: game_move })
    results.push(result)
    return result
  },
  createGame: async (agent, opponent) => {
    const result = await agent.callSync("main", "create_game", { opponent: opponent.agentId, timestamp: 0 })
    results.push(result)
    return result.Ok
  },
  renderState: async (agent, game_address) => {
    const result = await agent.callSync("main", "render_state", { game_address })
    console.log(result.Ok)
  },
  getState: async (agent, game_address) => {
    const result = await agent.callSync("main", "get_state", { game_address })
    results.push(result)
    return result
  },
}
