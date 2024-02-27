<script lang="ts">
  import "./newStyle.css";
  import { invoke } from "@tauri-apps/api/tauri";
  import { appWindow, WebviewWindow } from "@tauri-apps/api/window";
  import Race from "./racestap.svelte";

  var Stap = "stap 1";
</script>

<!-- TODO Speicher Vortschrit Irgendwo-->
<!-- TODO Speicher Character Daten Irgendwo-->
<!-- TODO Vüge Restliche Scheiße ein-->
<!-- TODO Alles Schönmachen  -->
<!-- TODO Testen  -->

<div class="status-anzeige">
  <p class="stap">{Stap}</p>
</div>

{#if Stap == "stap 0"}
  Beschreibung was kommt
{:else if Stap == "stap 1"}
  <!-- TODO  Choos your Character Race-->
  <Race />
{:else if Stap == "stap 2"}{:else if Stap == "stap 3"}

{/if}
