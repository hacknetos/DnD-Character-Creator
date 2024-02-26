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
      <!--TODO Stype and List all Persrks of The Races on Screen   -->
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
          <hr />
          <div class="race-extrapoints">
            <div class="race-speed">
              Speed: {all_known_races[selcetet_index].speed}
            </div>
            <div class="race-size">
              Height :
              {all_known_races[selcetet_index].standard_size.min} -
              {all_known_races[selcetet_index].standard_size.max}
              {#if all_known_races[selcetet_index].standard_size.description != ""}
                ({all_known_races[selcetet_index].standard_size.description})
              {/if}
              <br />
              weight: {all_known_races[selcetet_index].standard_weight}
            </div>
            <div class="race-age">
              age: {all_known_races[selcetet_index].standard_age[0]} -
              {all_known_races[selcetet_index].standard_age[1]} years
            </div>
            <div class="race-lang">
              languages :
              {#each all_known_races[selcetet_index].languages as lang}
                {lang + " "}
              {/each}
            </div>
            <div class="race-bonus">
              <details>
                <summary>Bonus:</summary>
                <ul class="race-bonus-list">
                  {#each all_known_races[selcetet_index].bonuses as bonus}
                    <li>{bonus.name} : {bonus.bonus}</li>
                  {/each}
                </ul>
              </details>
            </div>
            <div class="race-res">
              {#if all_known_races[selcetet_index].resistance != null}
                resistance:
                {#each all_known_races[selcetet_index].resistance as res}
                  {res}
                {/each}
              {/if}
            </div>
            <div class="race-skill">
              {#if all_known_races[selcetet_index].race_skills != null}
                <details>
                  <summary>Skill:</summary>
                  {#each all_known_races[selcetet_index].race_skills as skills}
                    <h4>{skills.name} :</h4>
                    <div class="race-skill-des">{skills.description}</div>
                  {/each}
                </details>
              {/if}
            </div>
            <div class="race-under-species">
              {#if all_known_races[selcetet_index].under_species != null}
                <details>
                  <summary>under species:</summary>

                  {#each all_known_races[selcetet_index].under_species as urace}
                    <details class="underrace-detail">
                      <summary>{urace.name}</summary>
                      <h4>description:</h4>
                      {urace.description}
                      {#if urace.under_race_bonus != null}
                        <details class="under-race-bon">
                          <summary>under race Bonus</summary>
                          <ul>
                            {#each urace.under_race_bonus as uracebon}
                              <li>{uracebon.name} : {uracebon.bonus}</li>
                            {/each}
                          </ul>
                        </details>
                      {/if}
                    </details>
                  {/each}
                </details>
              {/if}
            </div>
          </div>
        {/if}
      </div>
      <div class="option-row">Option Row</div>
    </div>
  </div>
{:else if Stap == "stap 2"}{:else if Stap == "stap 3"}

{/if}
