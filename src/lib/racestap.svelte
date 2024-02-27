<script lang="ts">
  import "./newStyle.css";
  import { invoke } from "@tauri-apps/api/tauri";
  import { appWindow, WebviewWindow } from "@tauri-apps/api/window";

  //TODO Safe all Theat in a Store
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
            {all_known_races[selcetet_index].standard_size.min}cm -
            {all_known_races[selcetet_index].standard_size.max}cm
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
                  <div class="race-skill-des">
                    <p>{skills.description}</p>
                  </div>
                {/each}
              </details>
              <br />
            {/if}
          </div>
          <div class="race-under-species">
            {#if all_known_races[selcetet_index].under_species != null}
              <details>
                <summary>under species:</summary>
                <ul class="under-race-ul">
                  {#each all_known_races[selcetet_index].under_species as urace}
                    <li class="under-race-li">
                      <details class="underrace-detail">
                        <summary>{urace.name}</summary>
                        <h4>description:</h4>
                        <p>{urace.description}</p>
                        {#if urace.under_race_size != null}
                          {urace.under_race_size.min}cm -
                          {urace.under_race_size.max}cm
                          {#if urace.under_race_size.description != ""}
                            ({urace.under_race_size.description})
                          {/if}
                        {/if}
                        <br />
                        {#if urace.under_race_age != null}
                          {urace.under_race_age[0]} years -
                          {urace.under_race_age[1]} years
                        {/if}
                        {#if urace.under_race_bonus != null}
                          <ul class="under-race-ul">
                            <li class="under-race-li">
                              <details class="under-race-bon">
                                <summary>under race Bonus</summary>
                                <ul class="under-race-ul">
                                  {#each urace.under_race_bonus as uracebon}
                                    <li class="under-race-li">
                                      {uracebon.name} : {uracebon.bonus}
                                    </li>
                                  {/each}
                                </ul>
                              </details>
                            </li>
                          </ul>
                          <br />
                        {/if}
                        {#if urace.under_race_resistance != null}
                          <ul class="under-race-ul">
                            <li class="under-race-li">
                              <details>
                                <summary>Extra Resistance</summary>
                                <ul class="under-race-ul">
                                  {#each urace.under_race_resistance as uracereses}
                                    <li class="under-race-li">{uracereses}</li>
                                  {/each}
                                </ul>
                              </details>
                            </li>
                          </ul>
                          <br />
                        {/if}
                        {#if urace.under_race_lang != null}
                          <ul class="under-race-ul">
                            <li class="under-race-li">
                              <details>
                                <summary>Language</summary>
                                <ul class="under-race-ul">
                                  {#each urace.under_race_lang as uracelang}
                                    <li class="under-race-li">{uracelang}</li>
                                  {/each}
                                </ul>
                              </details>
                            </li>
                          </ul>
                        {/if}
                        {#if urace.under_race_skills != null}
                          <ul class="under-race-ul">
                            <li class="under-race-li">
                              <details class="underrace-skills-wrapper">
                                <summary>Extra Skills</summary>
                                <ul class="under-race-ul">
                                  {#each urace.under_race_skills as uraceSkill}
                                    <li class="under-race-li">
                                      <details class="underrace-skills">
                                        <summary>{uraceSkill.name}</summary>
                                        <p>{uraceSkill.description}</p>
                                      </details>
                                    </li>
                                    <br />
                                  {/each}
                                </ul>
                              </details>
                            </li>
                          </ul>
                          <br />
                        {/if}
                      </details>
                    </li>
                    <br />
                  {/each}
                </ul>
              </details>
            {/if}
          </div>
        </div>
      {/if}
    </div>
    <div class="option-row">Option Row</div>
  </div>
</div>
