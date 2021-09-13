<script>
  import DataTable, { Head, Body, Row, Cell } from "@smui/data-table";
  import Checkbox from "@smui/checkbox";

  $: SelectedCount = selected.length;
  $: DeviceCount = devices.length;

  let devices = [
    {
      id: "RPi-1",
      type: "RaspberryPi",
      description: "Raspberry Pi A313D4",
      os: "Unbuntu",
      version: "r1.4",
    },
    {
      id: "RPi-2",
      type: "RaspberryPi",
      description: "John's Rpi.",
      os: "Unbuntu",
      version: "r1.0",
    },
    {
      id: "RPi-3",
      type: "RaspberryPi",
      description: "Roger's Pi",
      os: "Unbuntu",
      version: "r1.3",
    },
    {
      id: "RPi-4",
      type: "RaspberryPi",
      description: "The Pi with no name",
      os: "Unbuntu",
      version: "r2.0",
    },
    {
      id: "IPhone-ABC",
      type: "Phone",
      description: "Fanboy's iphone 13",
      os: "IOS",
      version: "r12.2",
    },
  ];
  let selected = [devices[2]];
</script>

<DataTable style="max-width: 100%; min-width: 100%">
  <Head>
    <Row>
      <Cell checkbox>
        <Checkbox />
      </Cell>
      <Cell>Device ID</Cell>
      <Cell>Type</Cell>
      <Cell>Description</Cell>
      <Cell>O/S</Cell>
      <Cell>Version</Cell>
    </Row>
  </Head>
  <Body>
    {#each devices as device (device.id)}
      <Row>
        <Cell checkbox>
          <Checkbox bind:group={selected} value={device} valueKey={device.id} />
        </Cell>
        <Cell>{device.id}</Cell>
        <Cell>{device.type}</Cell>
        <Cell>{device.description}</Cell>
        <Cell>{device.os}</Cell>
        <Cell>{device.version}</Cell>
      </Row>
    {/each}
  </Body>
</DataTable>

<pre
  class="status">Selected: {selected.map(device => device.id).join(', ')}</pre>

<pre class="status">Selected Total: {SelectedCount}/{DeviceCount}</pre>
