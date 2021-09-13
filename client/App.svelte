<script>
  import Admin from "./Admin/Admin.svelte";
  import Help from "./Help/Help.svelte";
  import Devices from "./Devices/Devices.svelte";
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
    <Devices />
  {:else if active == "Help"}
    <Help />
  {/if}
</div>
