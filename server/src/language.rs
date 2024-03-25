use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub enum Language {
    None,
    #[serde(rename = "Angular Template")]
    AngularTemplate,
    #[serde(rename = "APL")]
    Apl,
    #[serde(rename = "ASN.1")]
    Asn1,
    #[serde(rename = "Asterisk")]
    Asterisk,
    #[serde(rename = "Brainfuck")]
    Brainfuck,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "C#")]
    CSharp,
    #[serde(rename = "C++")]
    CPlusPlus,
    #[serde(rename = "Clojure")]
    Clojure,
    #[serde(rename = "ClojureScript")]
    ClojureScript,
    #[serde(rename = "Closure Stylesheets (GSS)")]
    ClosureStylesheets,
    #[serde(rename = "CMake")]
    CMake,
    #[serde(rename = "Cobol")]
    Cobol,
    #[serde(rename = "CoffeeScript")]
    CoffeeScript,
    #[serde(rename = "Common Lisp")]
    CommonLisp,
    #[serde(rename = "CQL")]
    Cql,
    #[serde(rename = "Crystal")]
    Crystal,
    #[serde(rename = "CSS")]
    Css,
    #[serde(rename = "Cypher")]
    Cypher,
    #[serde(rename = "Cython")]
    Cython,
    #[serde(rename = "D")]
    D,
    #[serde(rename = "Dart")]
    Dart,
    #[serde(rename = "diff")]
    Diff,
    #[serde(rename = "Dockerfile")]
    Dockerfile,
    #[serde(rename = "DTD")]
    Dtd,
    #[serde(rename = "Dylan")]
    Dylan,
    #[serde(rename = "EBNF")]
    Ebnf,
    #[serde(rename = "ECL")]
    Ecl,
    #[serde(rename = "edn")]
    Edn,
    #[serde(rename = "Eiffel")]
    Eiffel,
    #[serde(rename = "Elm")]
    Elm,
    #[serde(rename = "Erlang")]
    Erlang,
    #[serde(rename = "Esper")]
    Esper,
    #[serde(rename = "F#")]
    FSharp,
    #[serde(rename = "Factor")]
    Factor,
    #[serde(rename = "FCL")]
    Fcl,
    #[serde(rename = "Forth")]
    Forth,
    #[serde(rename = "Fortran")]
    Fortran,
    #[serde(rename = "Gas")]
    Gas,
    #[serde(rename = "Gherkin")]
    Gherkin,
    #[serde(rename = "Go")]
    Go,
    #[serde(rename = "Groovy")]
    Groovy,
    #[serde(rename = "Haskell")]
    Haskell,
    #[serde(rename = "Haxe")]
    Haxe,
    #[serde(rename = "HTML")]
    Html,
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "HXML")]
    Hxml,
    #[serde(rename = "IDL")]
    Idl,
    #[serde(rename = "Java")]
    Java,
    #[serde(rename = "JavaScript")]
    JavaScript,
    #[serde(rename = "Jinja2")]
    Jinja2,
    #[serde(rename = "JSON")]
    Json,
    #[serde(rename = "JSON-LD")]
    JsonLD,
    #[serde(rename = "JSX")]
    Jsx,
    #[serde(rename = "Julia")]
    Julia,
    #[serde(rename = "Kotlin")]
    Kotlin,
    #[serde(rename = "LaTeX")]
    LaTeX,
    #[serde(rename = "LESS")]
    Less,
    #[serde(rename = "Liquid")]
    Liquid,
    #[serde(rename = "LiveScript")]
    LiveScript,
    #[serde(rename = "Lua")]
    Lua,
    #[serde(rename = "MariaDB SQL")]
    MariaDBSQL,
    #[serde(rename = "Markdown")]
    Markdown,
    #[serde(rename = "Mathematica")]
    Mathematica,
    #[serde(rename = "Mbox")]
    Mbox,
    #[serde(rename = "mIRC")]
    Mirc,
    #[serde(rename = "Modelica")]
    Modelica,
    #[serde(rename = "MS SQL")]
    MsSQL,
    #[serde(rename = "MscGen")]
    MscGen,
    #[serde(rename = "MsGenny")]
    MsGenny,
    #[serde(rename = "MUMPS")]
    Mumps,
    #[serde(rename = "MySQL")]
    MySQL,
    #[serde(rename = "Nginx")]
    Nginx,
    #[serde(rename = "NSIS")]
    Nsis,
    #[serde(rename = "NTriples")]
    NTriples,
    #[serde(rename = "Objective-C")]
    ObjectiveC,
    #[serde(rename = "Objective-C++")]
    ObjectiveCPlusPlus,
    #[serde(rename = "OCaml")]
    OCaml,
    #[serde(rename = "Octave")]
    Octave,
    #[serde(rename = "Oz")]
    Oz,
    #[serde(rename = "Pascal")]
    Pascal,
    #[serde(rename = "Perl")]
    Perl,
    #[serde(rename = "PGP")]
    Pgp,
    #[serde(rename = "PHP")]
    Php,
    #[serde(rename = "Pig")]
    Pig,
    #[serde(rename = "PLSQL")]
    PlSQL,
    #[serde(rename = "PostgreSQL")]
    PostgreSQL,
    #[serde(rename = "PowerShell")]
    PowerShell,
    #[serde(rename = "Properties files")]
    Properties,
    #[serde(rename = "ProtoBuf")]
    ProtoBuf,
    #[serde(rename = "Puppet")]
    Puppet,
    #[serde(rename = "Python")]
    Python,
    #[serde(rename = "Q")]
    Q,
    #[serde(rename = "R")]
    R,
    #[serde(rename = "RPM Changes")]
    RPMChanges,
    #[serde(rename = "RPM Spec")]
    RPMSpec,
    #[serde(rename = "Ruby")]
    Ruby,
    #[serde(rename = "Rust")]
    Rust,
    #[serde(rename = "SAS")]
    Sas,
    #[serde(rename = "Sass")]
    Sass,
    #[serde(rename = "Scala")]
    Scala,
    #[serde(rename = "Scheme")]
    Scheme,
    #[serde(rename = "SCSS")]
    Scss,
    #[serde(rename = "Shell")]
    Shell,
    #[serde(rename = "Sieve")]
    Sieve,
    #[serde(rename = "Smalltalk")]
    Smalltalk,
    #[serde(rename = "SML")]
    Sml,
    #[serde(rename = "Solr")]
    Solr,
    #[serde(rename = "SPARQL")]
    Sparql,
    #[serde(rename = "Spreadsheet")]
    Spreadsheet,
    #[serde(rename = "SQL")]
    Sql,
    #[serde(rename = "SQLite")]
    SQLite,
    #[serde(rename = "Squirrel")]
    Squirrel,
    #[serde(rename = "sTeX")]
    STeX,
    #[serde(rename = "Stylus")]
    Stylus,
    #[serde(rename = "Swift")]
    Swift,
    #[serde(rename = "SystemVerilog")]
    SystemVerilog,
    #[serde(rename = "Tcl")]
    Tcl,
    #[serde(rename = "Textile")]
    Textile,
    #[serde(rename = "TiddlyWiki")]
    TiddlyWiki,
    #[serde(rename = "Tiki wiki")]
    Tikiwiki,
    #[serde(rename = "TOML")]
    Toml,
    #[serde(rename = "Troff")]
    Troff,
    #[serde(rename = "TSX")]
    Tsx,
    #[serde(rename = "TTCN")]
    Ttcn,
    #[serde(rename = "TTCN_CFG")]
    TTCNCfg,
    #[serde(rename = "Turtle")]
    Turtle,
    #[serde(rename = "TypeScript")]
    TypeScript,
    #[serde(rename = "VB.NET")]
    VBNet,
    #[serde(rename = "VBScript")]
    VBScript,
    #[serde(rename = "Velocity")]
    Velocity,
    #[serde(rename = "Verilog")]
    Verilog,
    #[serde(rename = "VHDL")]
    Vhdl,
    #[serde(rename = "Vue")]
    Vue,
    #[serde(rename = "Web IDL")]
    WebIDL,
    #[serde(rename = "WebAssembly")]
    WebAssembly,
    #[serde(rename = "XML")]
    Xml,
    #[serde(rename = "XQuery")]
    XQuery,
    #[serde(rename = "Xù")]
    Xu,
    #[serde(rename = "Yacas")]
    Yacas,
    #[serde(rename = "YAML")]
    Yaml,
    #[serde(rename = "Z80")]
    Z80
}

impl Language {

    ///
    /// Converts the Language enum to a string
    /// 
    /// The string is the capitalized version of the enum variant
    /// 
    pub fn to_string(&self) -> String {
        match self {
            Language::None => "NONE".to_string(),
            Language::AngularTemplate => "ANGULARTEMPLATE".to_string(),
            Language::Apl => "APL".to_string(),
            Language::Asn1 => "ASN1".to_string(),
            Language::Asterisk => "ASTERISK".to_string(),
            Language::Brainfuck => "BRAINFUCK".to_string(),
            Language::C => "C".to_string(),
            Language::CSharp => "CSHARP".to_string(),
            Language::CPlusPlus => "CPLUSPLUS".to_string(),
            Language::Clojure => "CLOJURE".to_string(),
            Language::ClojureScript => "CLOJURESCRIPT".to_string(),
            Language::ClosureStylesheets => "CLOSURESTYLESHEETS".to_string(),
            Language::CMake => "CMAKE".to_string(),
            Language::Cobol => "COBOL".to_string(),
            Language::CoffeeScript => "COFFEESCRIPT".to_string(),
            Language::CommonLisp => "COMMONLISP".to_string(),
            Language::Cql => "CQL".to_string(),
            Language::Crystal => "CRYSTAL".to_string(),
            Language::Css => "CSS".to_string(),
            Language::Cypher => "CYPHER".to_string(),
            Language::Cython => "CYTHON".to_string(),
            Language::D => "D".to_string(),
            Language::Dart => "DART".to_string(),
            Language::Diff => "DIFF".to_string(),
            Language::Dockerfile => "DOCKERFILE".to_string(),
            Language::Dtd => "DTD".to_string(),
            Language::Dylan => "DYLAN".to_string(),
            Language::Ebnf => "EBNF".to_string(),
            Language::Ecl => "ECL".to_string(),
            Language::Edn => "EDN".to_string(),
            Language::Eiffel => "EIFFEL".to_string(),
            Language::Elm => "ELM".to_string(),
            Language::Erlang => "ERLANG".to_string(),
            Language::Esper => "ESPER".to_string(),
            Language::FSharp => "FSHARP".to_string(),
            Language::Factor => "FACTOR".to_string(),
            Language::Fcl => "FCL".to_string(),
            Language::Forth => "FORTH".to_string(),
            Language::Fortran => "FORTRAN".to_string(),
            Language::Gas => "GAS".to_string(),
            Language::Gherkin => "GHERKIN".to_string(),
            Language::Go => "GO".to_string(),
            Language::Groovy => "GROOVY".to_string(),
            Language::Haskell => "HASKELL".to_string(),
            Language::Haxe => "HAXE".to_string(),
            Language::Html => "HTML".to_string(),
            Language::Http => "HTTP".to_string(),
            Language::Hxml => "HXML".to_string(),
            Language::Idl => "IDL".to_string(),
            Language::Java => "JAVA".to_string(),
            Language::JavaScript => "JAVASCRIPT".to_string(),
            Language::Jinja2 => "JINJA2".to_string(),
            Language::Json => "JSON".to_string(),
            Language::JsonLD => "JSONLD".to_string(),
            Language::Jsx => "JSX".to_string(),
            Language::Julia => "JULIA".to_string(),
            Language::Kotlin => "KOTLIN".to_string(),
            Language::LaTeX => "LATEX".to_string(),
            Language::Less => "LESS".to_string(),
            Language::Liquid => "LIQUID".to_string(),
            Language::LiveScript => "LIVESCRIPT".to_string(),
            Language::Lua => "LUA".to_string(),
            Language::MariaDBSQL => "MARIADBSQL".to_string(),
            Language::Markdown => "MARKDOWN".to_string(),
            Language::Mathematica => "MATHEMATICA".to_string(),
            Language::Mbox => "MBOX".to_string(),
            Language::Mirc => "MIRC".to_string(),
            Language::Modelica => "MODELICA".to_string(),
            Language::MsSQL => "MSSQL".to_string(),
            Language::MscGen => "MSCGEN".to_string(),
            Language::MsGenny => "MSGENNY".to_string(),
            Language::Mumps => "MUMPS".to_string(),
            Language::MySQL => "MYSQL".to_string(),
            Language::Nginx => "NGINX".to_string(),
            Language::Nsis => "NSIS".to_string(),
            Language::NTriples => "NTRIPLES".to_string(),
            Language::ObjectiveC => "OBJECTIVEC".to_string(),
            Language::ObjectiveCPlusPlus => "OBJECTIVECPLUSPLUS".to_string(),
            Language::OCaml => "OCAML".to_string(),
            Language::Octave => "OCTAVE".to_string(),
            Language::Oz => "OZ".to_string(),
            Language::Pascal => "PASCAL".to_string(),
            Language::Perl => "PERL".to_string(),
            Language::Pgp => "PGP".to_string(),
            Language::Php => "PHP".to_string(),
            Language::Pig => "PIG".to_string(),
            Language::PlSQL => "PLSQL".to_string(),
            Language::PostgreSQL => "POSTGRESQL".to_string(),
            Language::PowerShell => "POWERSHELL".to_string(),
            Language::Properties => "PROPERTIES".to_string(),
            Language::ProtoBuf => "PROTOBUF".to_string(),
            Language::Puppet => "PUPPET".to_string(),
            Language::Python => "PYTHON".to_string(),
            Language::Q => "Q".to_string(),
            Language::R => "R".to_string(),
            Language::RPMChanges => "RPMCHANGES".to_string(),
            Language::RPMSpec => "RPMSPEC".to_string(),
            Language::Ruby => "RUBY".to_string(),
            Language::Rust => "RUST".to_string(),
            Language::Sas => "SAS".to_string(),
            Language::Sass => "SASS".to_string(),
            Language::Scala => "SCALA".to_string(),
            Language::Scheme => "SCHEME".to_string(),
            Language::Scss => "SCSS".to_string(),
            Language::Shell => "SHELL".to_string(),
            Language::Sieve => "SIEVE".to_string(),
            Language::Smalltalk => "SMALLTALK".to_string(),
            Language::Sml => "SML".to_string(),
            Language::Solr => "SOLR".to_string(),
            Language::Sparql => "SPARQL".to_string(),
            Language::Spreadsheet => "SPREADSHEET".to_string(),
            Language::Sql => "SQL".to_string(),
            Language::SQLite => "SQLITE".to_string(),
            Language::Squirrel => "SQUIRREL".to_string(),
            Language::STeX => "STEX".to_string(),
            Language::Stylus => "STYLUS".to_string(),
            Language::Swift => "SWIFT".to_string(),
            Language::SystemVerilog => "SYSTEMVERILOG".to_string(),
            Language::Tcl => "TCL".to_string(),
            Language::Textile => "TEXTILE".to_string(),
            Language::TiddlyWiki => "TIDDLYWIKI".to_string(),
            Language::Tikiwiki => "TIKIWIKI".to_string(),
            Language::Toml => "TOML".to_string(),
            Language::Troff => "TROFF".to_string(),
            Language::Tsx => "TSX".to_string(),
            Language::Ttcn => "TTCN".to_string(),
            Language::TTCNCfg => "TTCNCFG".to_string(),
            Language::Turtle => "TURTLE".to_string(),
            Language::TypeScript => "TYPESCRIPT".to_string(),
            Language::VBNet => "VBNET".to_string(),
            Language::VBScript => "VBSCRIPT".to_string(),
            Language::Velocity => "VELOCITY".to_string(),
            Language::Verilog => "VERILOG".to_string(),
            Language::Vhdl => "VHDL".to_string(),
            Language::Vue => "VUE".to_string(),
            Language::WebIDL => "WEBIDL".to_string(),
            Language::WebAssembly => "WEBASSEMBLY".to_string(),
            Language::Xml => "XML".to_string(),
            Language::XQuery => "XQUERY".to_string(),
            Language::Xu => "XU".to_string(),
            Language::Yacas => "YACAS".to_string(),
            Language::Yaml => "YAML".to_string(),
            Language::Z80 => "Z80".to_string(),
        }
    }

    ///
    /// Converts a string to a Language enum
    /// 
    /// The string is the capitalized version of the enum variant.
    /// The same as the to_string method.
    ///
    pub fn from_string(s: &str) -> Option<Language> {
        match s {
            "NONE" => Some(Language::None),
            "ANGULARTEMPLATE" => Some(Language::AngularTemplate),
            "APL" => Some(Language::Apl),
            "ASN1" => Some(Language::Asn1),
            "ASTERISK" => Some(Language::Asterisk),
            "BRAINFUCK" => Some(Language::Brainfuck),
            "C" => Some(Language::C),
            "CSHARP" => Some(Language::CSharp),
            "CPLUSPLUS" => Some(Language::CPlusPlus),
            "CLOJURE" => Some(Language::Clojure),
            "CLOJURESCRIPT" => Some(Language::ClojureScript),
            "CLOSURESTYLESHEETS" => Some(Language::ClosureStylesheets),
            "CMAKE" => Some(Language::CMake),
            "COBOL" => Some(Language::Cobol),
            "COFFEESCRIPT" => Some(Language::CoffeeScript),
            "COMMONLISP" => Some(Language::CommonLisp),
            "CQL" => Some(Language::Cql),
            "CRYSTAL" => Some(Language::Crystal),
            "CSS" => Some(Language::Css),
            "CYPHER" => Some(Language::Cypher),
            "CYTHON" => Some(Language::Cython),
            "D" => Some(Language::D),
            "DART" => Some(Language::Dart),
            "DIFF" => Some(Language::Diff),
            "DOCKERFILE" => Some(Language::Dockerfile),
            "DTD" => Some(Language::Dtd),
            "DYLAN" => Some(Language::Dylan),
            "EBNF" => Some(Language::Ebnf),
            "ECL" => Some(Language::Ecl),
            "EDN" => Some(Language::Edn),
            "EIFFEL" => Some(Language::Eiffel),
            "ELM" => Some(Language::Elm),
            "ERLANG" => Some(Language::Erlang),
            "ESPER" => Some(Language::Esper),
            "FSHARP" => Some(Language::FSharp),
            "FACTOR" => Some(Language::Factor),
            "FCL" => Some(Language::Fcl),
            "FORTH" => Some(Language::Forth),
            "FORTRAN" => Some(Language::Fortran),
            "GAS" => Some(Language::Gas),
            "GHERKIN" => Some(Language::Gherkin),
            "GO" => Some(Language::Go),
            "GROOVY" => Some(Language::Groovy),
            "HASKELL" => Some(Language::Haskell),
            "HAXE" => Some(Language::Haxe),
            "HTML" => Some(Language::Html),
            "HTTP" => Some(Language::Http),
            "HXML" => Some(Language::Hxml),
            "IDL" => Some(Language::Idl),
            "JAVA" => Some(Language::Java),
            "JAVASCRIPT" => Some(Language::JavaScript),
            "JINJA2" => Some(Language::Jinja2),
            "JSON" => Some(Language::Json),
            "JSONLD" => Some(Language::JsonLD),
            "JSX" => Some(Language::Jsx),
            "JULIA" => Some(Language::Julia),
            "KOTLIN" => Some(Language::Kotlin),
            "LATEX" => Some(Language::LaTeX),
            "LESS" => Some(Language::Less),
            "LIQUID" => Some(Language::Liquid),
            "LIVESCRIPT" => Some(Language::LiveScript),
            "LUA" => Some(Language::Lua),
            "MARIADBSQL" => Some(Language::MariaDBSQL),
            "MARKDOWN" => Some(Language::Markdown),
            "MATHEMATICA" => Some(Language::Mathematica),
            "MBOX" => Some(Language::Mbox),
            "MIRC" => Some(Language::Mirc),
            "MODELICA" => Some(Language::Modelica),
            "MSSQL" => Some(Language::MsSQL),
            "MSCGEN" => Some(Language::MscGen),
            "MSGENNY" => Some(Language::MsGenny),
            "MUMPS" => Some(Language::Mumps),
            "MYSQL" => Some(Language::MySQL),
            "NGINX" => Some(Language::Nginx),
            "NSIS" => Some(Language::Nsis),
            "NTRIPLES" => Some(Language::NTriples),
            "OBJECTIVEC" => Some(Language::ObjectiveC),
            "OBJECTIVECPLUSPLUS" => Some(Language::ObjectiveCPlusPlus),
            "OCAML" => Some(Language::OCaml),
            "OCTAVE" => Some(Language::Octave),
            "OZ" => Some(Language::Oz),
            "PASCAL" => Some(Language::Pascal),
            "PERL" => Some(Language::Perl),
            "PGP" => Some(Language::Pgp),
            "PHP" => Some(Language::Php),
            "PIG" => Some(Language::Pig),
            "PLSQL" => Some(Language::PlSQL),
            "POSTGRESQL" => Some(Language::PostgreSQL),
            "POWERSHELL" => Some(Language::PowerShell),
            "PROPERTIES" => Some(Language::Properties),
            "PROTOBUF" => Some(Language::ProtoBuf),
            "PUPPET" => Some(Language::Puppet),
            "PYTHON" => Some(Language::Python),
            "Q" => Some(Language::Q),
            "R" => Some(Language::R),
            "RPMCHANGES" => Some(Language::RPMChanges),
            "RPMSPEC" => Some(Language::RPMSpec),
            "RUBY" => Some(Language::Ruby),
            "RUST" => Some(Language::Rust),
            "SAS" => Some(Language::Sas),
            "SASS" => Some(Language::Sass),
            "SCALA" => Some(Language::Scala),
            "SCHEME" => Some(Language::Scheme),
            "SCSS" => Some(Language::Scss),
            "SHELL" => Some(Language::Shell),
            "SIEVE" => Some(Language::Sieve),
            "SMALLTALK" => Some(Language::Smalltalk),
            "SML" => Some(Language::Sml),
            "SOLR" => Some(Language::Solr),
            "SPARQL" => Some(Language::Sparql),
            "SPREADSHEET" => Some(Language::Spreadsheet),
            "SQL" => Some(Language::Sql),
            "SQLITE" => Some(Language::SQLite),
            "SQUIRREL" => Some(Language::Squirrel),
            "STEX" => Some(Language::STeX),
            "STYLUS" => Some(Language::Stylus),
            "SWIFT" => Some(Language::Swift),
            "SYSTEMVERILOG" => Some(Language::SystemVerilog),
            "TCL" => Some(Language::Tcl),
            "TEXTILE" => Some(Language::Textile),
            "TIDDLYWIKI" => Some(Language::TiddlyWiki),
            "TIKIWIKI" => Some(Language::Tikiwiki),
            "TOML" => Some(Language::Toml),
            "TROFF" => Some(Language::Troff),
            "TSX" => Some(Language::Tsx),
            "TTCN" => Some(Language::Ttcn),
            "TTCNCFG" => Some(Language::TTCNCfg),
            "TURTLE" => Some(Language::Turtle),
            "TYPESCRIPT" => Some(Language::TypeScript),
            "VBNET" => Some(Language::VBNet),
            "VBSCRIPT" => Some(Language::VBScript),
            "VELOCITY" => Some(Language::Velocity),
            "VERILOG" => Some(Language::Verilog),
            "VHDL" => Some(Language::Vhdl),
            "VUE" => Some(Language::Vue),
            "WEBIDL" => Some(Language::WebIDL),
            "WEBASSEMBLY" => Some(Language::WebAssembly),
            "XML" => Some(Language::Xml),
            "XQUERY" => Some(Language::XQuery),
            "XU" => Some(Language::Xu),
            "YACAS" => Some(Language::Yacas),
            "YAML" => Some(Language::Yaml),
            "Z80" => Some(Language::Z80),
            _ => None
        }
    }
}