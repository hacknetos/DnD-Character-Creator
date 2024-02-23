<script lang="ts">
  import "./newStyle.css";
  import { invoke } from "@tauri-apps/api/tauri";
  import { appWindow, WebviewWindow } from "@tauri-apps/api/window";

  var Stap = "stap 1";
  let all_known_races = [];
  let selectet_race = "";
  async function get_all_known_races() {
    all_known_races = await invoke("get_race");
    console.log(all_known_races);
  }
  let selcetet_index = -1;
  get_all_known_races();
</script>

<div class="status-anzeige">
  <p class="stap">{Stap}</p>
</div>

{#if Stap == "stap 0"}
  Beschreibung was kommt
{:else if Stap == "stap 1"}
  <div transition:fade={{ duration: 250 }} class="rapper">
    <div class="header-div">
      <h1>Choose a race</h1>
    </div>
    <div class="selcter-race-class-grid">
      <div class="race-class-lister">
        <ul class="RaceList">
          {#each all_known_races as race, i}
            <li class="RaceList-elements">
              <input
                type="radio"
                class="RaceList-radio"
                name="raceSelecterRadio"
                on:click={() => {
                  selectet_race = race.name;
                  selcetet_index = i;
                  console.log(race.name + " " + i);
                }}
              />
              {race.name}
            </li>
          {/each}
        </ul>
      </div>
      <div class="race-class-description">
        {#if selcetet_index == -1}
          <p>Pleas Select a Race</p>
        {:else}
          <div class="race-description">
            <h4 class="race-description-head">
              Description {all_known_races[selcetet_index].name}
            </h4>
            {all_known_races[selcetet_index].description}
          </div>
          <div>{all_known_races[selcetet_index].speed}</div>
        {/if}
      </div>
      <div class="option-row">Option Row</div>
    </div>
  </div>
{:else if Stap == "stap 2"}{:else if Stap == "stap 3"}

{/if}
