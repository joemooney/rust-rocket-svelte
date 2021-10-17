<script>
  import { lineIterator } from "reader-line-iterator/src/line-iterator.mjs";
  import { LogView } from "svelte-log-view/src/index.svelte";
  import { request } from "../Server/Request.svelte";

  // import Request from "../Server/Request.svelte";
  let url = "/admin/journalctl/0/5";
  let number = 5;
  // let promise = request(url);
  function handleClick() {
    // promise = request(url);
    console.log("Trigger Starting log viewer");
    start = 1;
    follow = true;
  }
  // Built-in Browser component allows you to abort a Web request
  let controller = new AbortController();

  // source is a function that provides lines to view
  // it has a fetch function which is a generator.
  // when the rows that it has are exhausted then
  const source = {
    abort: async () => controller.abort(),
    fetch: async function* (cursor, offset, number) {
      console.log("Journal.svelte: fetch called ", offset, ":", number);
      //if (controller) {
      //  console.log("Abort log viewer");
      //  controller.abort();
      //}
      //controller = new AbortController();
      const params = {
        offset,
        number,
      };
      // cursor is the current line number
      // it is
      if (cursor) {
        params.cursor = cursor.substring(5);
      } else {
        params.cursor = 0;
      }
      console.log("Journal.svelte: trying fetch");
      try {
        const response = await fetch(
          //`/admin/journalctl?${new URLSearchParams(Object.entries(params))}`,
          `/admin/journalctl/${params.cursor}/${offset}/${number}`,
          {
            //signal: controller.signal,
          }
        );
        console.log("Journal.svelte: awaited fetch ");
        yield* lineIterator(response.body.getReader());
      } catch (e) {
        console.log("Journal.svelte: Error in log viewer", e);
        if (!e instanceof AbortSignal) {
          throw e;
        }
      }
    },
  };
  let start = 0;
  let follow = true;
  let selected = -1;

  /*
  async function* makeTextFileLineIterator(fileURL) {
    const utf8Decoder = new TextDecoder("utf-8");
    console.log("Journal.svelte: fetching url ", fileURL);
    const response = await fetch(fileURL);
    const reader = response.body.getReader();
    let { value: chunk, done: readerDone } = await reader.read();
    console.log("Journal.svelte: got ", chunk);
    chunk = chunk ? utf8Decoder.decode(chunk) : "";
    console.log("Journal.svelte: GOT ", chunk);

    const re = /\n|\r|\r\n/gm;
    let startIndex = 0;
    let result;

    for (;;) {
      let result = re.exec(chunk);
      if (!result) {
        if (readerDone) {
          break;
        }
        let remainder = chunk.substr(startIndex);
        ({ value: chunk, done: readerDone } = await reader.read());
        chunk = remainder + (chunk ? utf8Decoder.decode(chunk) : "");
        startIndex = re.lastIndex = 0;
        continue;
      }
      yield chunk.substring(startIndex, result.index);
      startIndex = re.lastIndex;
    }
    if (startIndex < chunk.length) {
      // last line didn't end in a newline char
      yield chunk.substr(startIndex);
    }
  }

  async function run() {
    console.log("Journal.svelte: getting url");
    let urlOfFile = `/admin/journalctl2`;
    for await (let line of makeTextFileLineIterator(urlOfFile)) {
      //processLine(line);
      console.log(line);
    }
  }

  function handleClick2() {
    run();
  }
  <button on:click={handleClick2}>Start2</button>
  */
</script>

<main>
  <h1>Journal</h1>

  <div id="log">
    <LogView
      visibleRows={10}
      {source}
      let:entry
      let:position
      bind:selected
      bind:follow
      bind:start
    >
      <div class={selected === position ? "selected" : ""}>{entry}</div>
    </LogView>
  </div>
  <p>
    start:{start} | selected: {selected} | follow: {follow ? "True" : "False"}
  </p>
  <button on:click={handleClick}>Start</button>
</main>
