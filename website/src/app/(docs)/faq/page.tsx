import Image from "next/image";
import Link from "next/link";
import { Hint } from "@/app/(utils)/Hint";
import { Highlight } from "@/app/(utils)/Highlight";
import { Figures } from "@/app/(utils)/Figures";
import ScreenshotFileNesting from "./figures/screenshot-file-nesting.png";

export default function GettingStarted() {
  return (
    <main>
      <h2>Frequently Asked Questions</h2>

      <h3>Why did you create nitrogql?</h3>
      <p>
        I needed a GraphQL + TypeScript tool for schema-based approach with
        source maps support.
      </p>

      <h3>Does nitrogql work with any UI library or framework?</h3>
      <p>
        Yes. Thanks to TypedDocumentNode, generated types are independent of UI
        libraries or frameworks. For the same reason, you can use nitrogql with
        any GraphQL client.
      </p>

      <h3>Is nitrogql fast?</h3>
      <p>
        Yes. nitrogql is written in Rust and is compiled to WebAssembly, running
        on Node.js&apos; WASM runtime. Often Node.js startup time is longer than
        the time it takes to generate types.
      </p>

      <h3>Does nitrogql support GraphQL code embedded in TypeScript?</h3>
      <p>
        No. I like to keep my GraphQL code separate from TypeScript code. I
        think this is the most straightforward approach to generating types from
        GraphQL code.
      </p>
      <p>
        Also, from a tool author perspective, supporting embedded GraphQL code
        complicates the parser.
      </p>

      <h3>
        I don&apos;t like generated files scattered among my source files.
      </h3>
      <p>
        If you use VS Code, you can use the <b>file nesting</b> feature to
        collapse generated files. Here is an example setting:
      </p>
      <Highlight language="json">
        {`{
  "explorer.fileNesting.enabled": true,
  "explorer.fileNesting.patterns": {
    "*.graphql": "\${capture}.d.graphql.ts,\${capture}.d.graphql.ts.map"
  }
}`}
      </Highlight>
      <Figures>
        <figure>
          <Image
            src={ScreenshotFileNesting}
            width="640"
            alt="Screenshot of VSCode file nesting feature"
          />
          <figcaption>
            Generated files can be collapsed under the original GraphQL file.
          </figcaption>
        </figure>
      </Figures>
      <p>
        As another option, nitrogql could implement a mode that generates all
        types in a dedicated directory. Please let us know if you would like to
        see this feature.
      </p>

      <h3>I need a watch mode!</h3>
      <p>
        Yes, watch mode is definitely a nice feature to have, although it is
        given a lower priority than other features. For now, if you use VS Code,
        you can use the{" "}
        <b>
          <a href="https://marketplace.visualstudio.com/items?itemName=emeraldwalk.RunOnSave">
            Run on Save
          </a>
        </b>{" "}
        extension to run nitrogql on save. It should be fast enough.
      </p>
      <p>Here is an example setting:</p>
      <Highlight language="json">
        {`{
  "emeraldwalk.runonsave": {
    "commands": [
      {
        "match": "\\\\.graphql$",
        "cmd": "npx nitrogql generate"
      }
    ]
  }
}`}
      </Highlight>

      <h3>Contribution?</h3>
      <p>
        <a href="https://github.com/uhyo/nitrogql">Welcome!</a>
      </p>

      <h3>Why is the nitrogql logo so crooked?</h3>
      <p>It&apos; drawn by an AI.</p>
    </main>
  );
}
