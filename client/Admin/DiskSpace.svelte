<script>
  import { request } from "../Server/Request.svelte";

  // import Request from "../Server/Request.svelte";
  let url = "/admin/diskspace";
  let promise = request(url);
  function handleClick() {
    promise = request(url);
  }
</script>

<main>
  <h1>Disk Space</h1>
  {#await promise}
    <p>...Retrieving disk space from Rust (with a 1 second sleep)</p>
  {:then text}
    <p>Message from Rust: {text}</p>
  {:catch error}
    <p style="color: red">{error.message}</p>
  {/await}
  <button on:click={handleClick}>Refresh</button>
</main>
