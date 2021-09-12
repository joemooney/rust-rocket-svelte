<script context="module">
  //export let promise = "EMPTY_PROMISE";
  //export let url = "PLEASE_SET_URL";
  export function sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }
  export async function request(url) {
    console.log("getting server Info...");
    const res = await fetch(url);
    await sleep(1000);
    console.log("getting server info...");
    // const text = await res.json();
    const text = await res.text();
    console.log("got server json...");

    if (res.ok) {
      console.log("got server ok...", text);
      return text;
    } else {
      console.log("got server error...", text);
      throw new Error(text);
    }
  }
  /*
  // TODO: how to have promise and url exported?
  function handleRefresh() {
    //  //text = "calling";
    promise = request(url);
  }
  */

  //promise = getServerInfo();
</script>
