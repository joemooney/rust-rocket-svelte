<script>
  import Drawer, { AppContent, Content } from "@smui/drawer";
  import List, { Item, Text, Graphic } from "@smui/list";

  // Icons for Components (my convention, prefix with 'I' as in IComponent)
  import ICheck from "svelte-material-icons/Check.svelte";
  import ITransitConnectionVariant from "svelte-material-icons/TransitConnectionVariant.svelte";
  import IDiskSpace from "svelte-material-icons/Database.svelte";
  import IJournal from "svelte-material-icons/MathLog.svelte";
  import IUptime from "svelte-material-icons/Clock.svelte";
  //import IPowerCycle from "svelte-material-icons/PowerCycle.svelte";
  import IPowerCycle from "svelte-material-icons/Flash.svelte";
  import ILaunch from "svelte-material-icons/Rocket.svelte";
  import ISettings from "svelte-material-icons/WrenchOutline.svelte";
  import IReload from "svelte-material-icons/Reload.svelte";
  //import Plug from "svelte-material-icons/PowerPlug.svelte";
  //import PlugOff from "svelte-material-icons/PowerPlugOff.svelte";
  import IPerformance from "svelte-material-icons/ChartLine.svelte";
  import IStatus from "svelte-material-icons/ShieldCheckOutline.svelte";
  import IFoo from "svelte-material-icons/ChartLine.svelte";

  // Components
  import DiskSpace from "./DiskSpace.svelte";
  import Status from "./Status.svelte";

  let clicked = "";

  export let size = "2em";
  export let width = size;
  export let height = size;
  export let color = "gray";
  export let viewBox = "0 0 30 30";
</script>

<div class="drawer-container">
  <Drawer>
    <Content>
      <List>
        <Item
          href="javascript:void(0)"
          on:click={() => (clicked = "Disk Space")}
        >
          <IDiskSpace {size} {color} {width} {height} {viewBox} />
          <Text>Disk Space</Text>
        </Item>

        <Item href="javascript:void(0)" on:click={() => (clicked = "Journal")}>
          <IJournal {size} {color} {width} {height} {viewBox} />
          <Text>Journal</Text>
        </Item>

        <Item href="javascript:void(0)" on:click={() => (clicked = "Uptime")}>
          <IUptime {size} {color} {width} {height} {viewBox} />
          <Text>Uptime</Text>
        </Item>

        <Item
          href="javascript:void(0)"
          on:click={() => (clicked = "Power Cycle")}
        >
          <IPowerCycle {size} {color} {width} {height} {viewBox} />
          <Text>Power Cycle</Text>
        </Item>

        <Item
          href="javascript:void(0)"
          on:click={() => (clicked = "Launch Application")}
        >
          <ILaunch {size} {color} {width} {height} {viewBox} />
          <Text>Launch Application</Text>
        </Item>

        <Item href="javascript:void(0)" on:click={() => (clicked = "Reload")}>
          <IReload {size} {color} {width} {height} {viewBox} />
          <Text>Reload</Text>
        </Item>

        <Item href="javascript:void(0)" on:click={() => (clicked = "Settings")}>
          <ISettings {size} {color} {width} {height} {viewBox} />
          <Text>Settings</Text>
        </Item>

        <Item
          href="javascript:void(0)"
          on:click={() => (clicked = "Performance")}
        >
          <IPerformance {size} {color} {width} {height} {viewBox} />
          <Text>Performance</Text>
        </Item>

        <Item href="javascript:void(0)" on:click={() => (clicked = "Status")}>
          <IStatus {size} {color} {width} {height} {viewBox} />
          <Text>Status</Text>
        </Item>
      </List>
    </Content>
  </Drawer>

  <AppContent class="app-content">
    <main class="main-content">
      {#if clicked == "Disk Space"}
        <DiskSpace />
      {:else if clicked == "Status"}
        <Status />
      {:else if clicked == ""}
        Device Manager Control Panel
      {:else}
        <p>you clicked {clicked}</p>
      {/if}
    </main>
  </AppContent>
</div>

<style>
  /* These classes are only needed because the
      drawer is in a container on the page. */
  .drawer-container {
    position: relative;
    display: flex;
    /*height: 350px;*/
    /*max-width: 600px;*/
    border: 1px solid
      var(--mdc-theme-text-hint-on-background, rgba(0, 0, 0, 0.1));
    overflow: hidden;
    z-index: 0;
  }

  * :global(.app-content) {
    flex: auto;
    overflow: auto;
    position: relative;
    flex-grow: 1;
  }

  .main-content {
    overflow: auto;
    padding: 16px;
    height: 100%;
    box-sizing: border-box;
  }
</style>
