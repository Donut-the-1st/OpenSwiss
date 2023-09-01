<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import SvelteTable from "svelte-table";


    // define column configs
    const columns = [
        {
            key: "ID",
            title: "ID",
            value: v => v.ID,
        },
        {
            key: "player_name",
            title: "Name",
            value: v => v.player_name,
        },
        {
            key: "wins",
            title: "Wins",
            value: v => v.wins,
            sortable: true
        },
        {
            key: "losses",
            title: "Losses",
            value: v => v.losses,
            sortable: true
        },
        {
            key: "score",
            title: "Score",
            value: v => v.score,
            sortable: true
        },
    ];

    let player_1_id = ''
    let player_2_id = ''
    let player_1_wins = ''
    let player_2_wins = ''
    let rows = []


    async function init() {
        await invoke('initialise')
    }
    async function addResult() {
        await invoke('add_result', { result:`{"player_1_id":${player_1_id},"player_2_id":${player_2_id},"player_1_wins":${player_1_wins},"player_2_wins":${player_2_wins}}` })
        await getPlayers()
    }

    async function getPlayers() {
        rows = JSON.parse(await invoke('get_players'))
    }

    async function scoreUpdater() {
        await invoke('update_results')
        await invoke('update_player_scores')
        await getPlayers()
    }

    init()
    getPlayers()

</script>

<div>
    <input id="player-1" placeholder="Enter player 1..." bind:value="{player_1_id}" />
    <input id="player-2" placeholder="Enter player 2..." bind:value="{player_2_id}" />
    <input id="player-1w" placeholder="Enter player 1 wins..." bind:value="{player_1_wins}" />
    <input id="player-2w" placeholder="Enter player 2 wins..." bind:value="{player_2_wins}" />
    <button
            on:click="{addResult}"
            on:click="{getPlayers}"
    >
        Add Result
    </button>
    <button
            on:click="{scoreUpdater}"
            on:click="{getPlayers}"
    >
        Update Score
    </button>
    <SvelteTable columns="{columns}" rows="{rows}"></SvelteTable>
</div>