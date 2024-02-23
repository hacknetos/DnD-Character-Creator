<script lang="ts">
  import { fade } from "svelte/transition";
  import logo from "./lib/logo.gif";
  import NewCharacter from "./lib/New.svelte";
  var current = "All";
  var theactive = current;
  function setContent() {
    theactive = "";
    setTimeout(() => {
      theactive = current;
    }, 250);
  }
</script>

<main class="content">
  <nav class="navbar navbar-grid">
    <div class="navbar-logo">
      <img alt="App Logo" src={logo} class="navbar-logo-img" />
    </div>
    <div class="navbar-buttom-div">
      <button
        class="navbar-button"
        class:active={current == "New"}
        on:click={() => {
          current = "New";
          setContent();
        }}
      >
        New
      </button>
      <button
        class="navbar-button"
        class:active={current == "All"}
        on:click={() => {
          current = "All";
          setContent();
        }}
      >
        ALL
      </button>
      <button
        class="navbar-button"
        class:active={current == "Custem"}
        on:click={() => {
          current = "Custem";
          setContent();
        }}
      >
        Custem
      </button>
    </div>
  </nav>
  <div class="main-Content">
    {#if theactive == "New"}
      <div transition:fade={{ duration: 250 }}>
        <NewCharacter />
      </div>
    {:else if theactive == "All"}
      <div transition:fade={{ duration: 250 }}>All</div>
    {:else if theactive == "Custem"}
      <div transition:fade={{ duration: 250 }}>Custem</div>
    {/if}
  </div>
</main>
