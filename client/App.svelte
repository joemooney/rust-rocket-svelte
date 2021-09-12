<script>
  import Admin from "./Admin/Admin.svelte";
  import Tab, { Label } from "@smui/tab";
  import TabBar from "@smui/tab-bar";
  import Button from "@smui/button";
  let active = "Admin";
</script>

<link rel="stylesheet" href="./svelte-material-ui-bare.css" />

<div>
  <!--
    Note: tabs must be unique. (They cannot === each other.)
  -->
  <TabBar tabs={["Admin", "Devices", "Help"]} let:tab bind:active>
    <!-- Note: the `tab` property is required! -->
    <Tab {tab}>
      <Label>{tab}</Label>
    </Tab>
  </TabBar>

  {#if active == "Admin"}
    <Admin />
  {:else if active == "Devices"}
    <Admin />
  {:else if active == "Help"}
    <Admin />
  {/if}
</div>
