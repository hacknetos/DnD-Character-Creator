<script lang="ts">
  import "./newStyle.css";
  import { invoke } from "@tauri-apps/api/tauri";

  var Stap = "stap 1";
  let all_known_races = [];

  async function get_all_known_races() {
    all_known_races = await invoke("invoke_get_all_known_races");
    console.log(all_known_races);
  }
</script>

<div class="status-anzeige">
  <p class="stap">{Stap}</p>
</div>

{#if Stap == "stap 0"}
  Beschreibung was kommt
{:else if Stap == "stap 1"}
  {get_all_known_races()}
  <div transition:fade={{ duration: 250 }} class="rapper">
    <div class="header-div">
      <h1>Choose a race</h1>
    </div>
    <div class="selcter-race-class-grid">
      <div class="race-class-lister">list of Races</div>
      <div class="race-class-description">Beschrewibung und Bonuse</div>
      <div class="option-row">Option Row</div>
    </div>
  </div>
{:else if Stap == "stap 2"}{:else if Stap == "stap 3"}

{/if}
