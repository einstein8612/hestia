<script lang="ts">
    let { language }: {language: string} = $props();

    const LANGUAGES = [
        "Angular Template",
        "APL",
        "ASN.1",
        "Asterisk",
        "Brainfuck",
        "C",
        "C#",
        "C++",
        "Clojure",
        "ClojureScript",
        "Closure Stylesheets (GSS)",
        "CMake",
        "Cobol",
        "CoffeeScript",
        "Common Lisp",
        "CQL",
        "Crystal",
        "CSS",
        "Cypher",
        "Cython",
        "D",
        "Dart",
        "diff",
        "Dockerfile",
        "DTD",
        "Dylan",
        "EBNF",
        "ECL",
        "edn",
        "Eiffel",
        "Elm",
        "Erlang",
        "Esper",
        "F#",
        "Factor",
        "FCL",
        "Forth",
        "Fortran",
        "Gas",
        "Gherkin",
        "Go",
        "Groovy",
        "Haskell",
        "Haxe",
        "HTML",
        "HTTP",
        "HXML",
        "IDL",
        "Java",
        "JavaScript",
        "Jinja2",
        "JSON",
        "JSON-LD",
        "JSX",
        "Julia",
        "Kotlin",
        "LaTeX",
        "LESS",
        "Liquid",
        "LiveScript",
        "Lua",
        "MariaDB SQL",
        "Markdown",
        "Mathematica",
        "Mbox",
        "mIRC",
        "Modelica",
        "MS SQL",
        "MscGen",
        "MsGenny",
        "MUMPS",
        "MySQL",
        "Nginx",
        "NSIS",
        "NTriples",
        "Objective-C",
        "Objective-C++",
        "OCaml",
        "Octave",
        "Oz",
        "Pascal",
        "Perl",
        "PGP",
        "PHP",
        "Pig",
        "PLSQL",
        "PostgreSQL",
        "PowerShell",
        "Properties files",
        "ProtoBuf",
        "Puppet",
        "Python",
        "Q",
        "R",
        "RPM Changes",
        "RPM Spec",
        "Ruby",
        "Rust",
        "SAS",
        "Sass",
        "Scala",
        "Scheme",
        "SCSS",
        "Shell",
        "Sieve",
        "Smalltalk",
        "SML",
        "Solr",
        "SPARQL",
        "Spreadsheet",
        "SQL",
        "SQLite",
        "Squirrel",
        "sTeX",
        "Stylus",
        "Swift",
        "SystemVerilog",
        "Tcl",
        "Textile",
        "TiddlyWiki",
        "Tiki wiki",
        "TOML",
        "Troff",
        "TSX",
        "TTCN",
        "TTCN_CFG",
        "Turtle",
        "TypeScript",
        "VB.NET",
        "VBScript",
        "Velocity",
        "Verilog",
        "VHDL",
        "Vue",
        "Web IDL",
        "WebAssembly",
        "XML",
        "XQuery",
        "Xù",
        "Yacas",
        "YAML",
        "Z80"
    ]

    let dialog: HTMLDialogElement | undefined = $state();
    let term: string = $state("");

    const filter = (term: string, data: string[]): string[] => {
        return data.filter((item: string) => {
            return item.toLowerCase().includes(term.toLowerCase());
        });
    }

    const selectLanguage = (newLanguage: string) => {
        language = newLanguage;
        dialog?.close();
    }
    
    const openModal = () => {
        dialog?.showModal();
    }
</script>

<dialog bind:this={dialog}>
    <div class="language-selector-inner">
        <div class="language-selector-header">
            <h2>Language</h2>
            <input id="language-search" type="text" placeholder="Language" bind:value={term} />
        </div>
        <ul>
            {#each filter(term, LANGUAGES) as language}
            <li><button class="language" on:click={() => {selectLanguage(language)}}>{language}</button></li>
            {/each}
        </ul>
    </div>
</dialog>
<button id="language-button" on:click={openModal}>{language ? language : "Language"}</button>

<style>
    #language-button {
        cursor: pointer;
        border: none;
        outline: none;
        padding: 5px 10px;
        min-width: 150px;
        color: var(--primary-colour);
        background-color: var(--secondary-bg-colour);
        transition: 0.3s ease all;
        font-size: 1.2em;
    }

    #language-button:hover {
        background-color: rgba(255,255,255,0.1);
    }

    dialog {
        border: none;
        margin: 0 auto;
        max-height: 100vh;
        width: 100%;
        max-width: 750px;
    }

    ::backdrop {
        backdrop-filter: blur(8px);
    }

    .language-selector-inner {
        display: flex;
        flex-direction: column;
        height: 100vh;
        width: 100%;
        max-width: 750px;
        background-color: var(--secondary-bg-colour);
    }

    .language-selector-header {
        display: flex;
        justify-content: space-evenly;
        flex-wrap: wrap;
        gap: 10px;
        padding: 20px 40px;
    }

    h2 {
        color: var(--tertiary-colour);
    }

    #language-search {
        border: none;
        outline: none;
        border-bottom: 1px solid var(--primary-bg-colour);
        font-size: 1.2em;
        color: var(--tertiary-colour);
        background-color: transparent;
    }

    .language {
        position: relative;
        top: 0px;
        cursor: pointer;
        border: none;
        outline: none;
        padding: 10px 20px;
        height: 100%;
        width: 100%;
        color: var(--primary-colour);
        background-color: var(--secondary-bg-colour);
        transition: 0.3s ease all;
        font-size: 1.2em;
        text-align: start;
    }

    ul {
        background-color: var(--secondary-bg-colour);
    }

    li:not(:last-child) {
        border-bottom: 1px var(--primary-bg-colour) solid;
    }

    .language:hover {
        background-color: rgba(255,255,255,0.1);
    }
</style>