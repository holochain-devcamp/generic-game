
module.exports = (scenario)=> {
  scenario("Bob can accept Alices proposal, create a game and Alice can see the game", async (s, t, { alice, bob }) => {
    const addr = await alice.callSync("main", "create_proposal", {message : "sup"})
    t.equal(addr.Ok.length, 46, "Proposal was created successfully")

    const proposals = await bob.callSync("main", "get_proposals", {})
    console.log(proposals)
    t.equal(proposals.Ok.length, 1, "Bob could retrieve Alices Proposal")

    const acceptance = await bob.callSync("main", "accept_proposal", { proposal_addr: proposals.Ok[0].address, created_at: 0 })
    t.notEqual(acceptance.Ok, undefined, "Bob could accept the proposal by creating a game") // check it returned Ok

    const games = await bob.callSync("main", "check_responses", { proposal_addr: proposals.Ok[0].address })
    t.deepEqual(
      games.Ok, 
      [{ 
        entry: { 
          player_1: bob.agentId,
          player_2: alice.agentId,
          created_at: 0
        }, 
        address: games.Ok[0].address
      }],
      "The game was created as expected"
    )
  })
}
