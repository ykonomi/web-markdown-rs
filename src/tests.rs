use html_translator::top;

#[test]
fn test_example_1() {
    let input = "\tfoo\tbaz\t\tbim";
    let output = "<pre><code>foo\tbaz\t\tbim</code></pre>";

    assert_eq!(top(input), output);
}

#[test]
fn test_example_2() {
    let input = "  \tfoo\tbaz\t\tbim";
    let output = "<pre><code>foo\tbaz\t\tbim</code></pre>";

    assert_eq!(top(input), output);
}

#[test]
fn test_example_3() {
    let input = "    a\ta\n    ὐ\ta";
    let output = "<pre><code>a\ta\nὐ\ta</code></pre>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_13() {
    let input = "***\n---\n___";
    let output = "<hr /><hr /><hr />";

    assert_eq!(top(input), output);
}

#[test]
fn test_example_14() {
    let input = "+++";
    let output = "<p>+++</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_15() {
    let input = "===";
    let output = "<p>===</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_16() {
    let input = "--\n**\n__";
    let output = "<p>--\n**\n__</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_17() {
    let input = " ***\n  ***\n   ***";
    let output = "<hr /><hr /><hr />";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_18() {
    let input = "    ***";
    let output = "<pre><code>***</code></pre>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_19() {
    let input = "Foo\n    ***";
    let output = "<p>Foo\n***</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_20() {
    let input = "_____________________________________";
    let output = "<hr />";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_21() {
    let input = " - - -";
    let output = "<hr />";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_22() {
    let input = " **  * ** * ** * **";
    let output = "<hr />";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_23() {
    let input = "-     -      -      -";
    let output = "<hr />";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_24() {
    let input = "- - - -    ";
    let output = "<hr />";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_25() {
    let input = "_ _ _ _ a\n\na------\n\n---a---";
    let output = "<p>_ _ _ _ a</p><p>a------</p><p>---a---</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_26() {
    let input = "*-*";
    let output = "<p><em>-</em></p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_27() {
    let input = "- foo\n***\n- bar";
    let output = "<ul><li>foo</li></ul><hr /><ul><li>bar</li></ul>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_28() {
    let input = "Foo\n***\nbar";
    let output = "<p>Foo</p><hr /><p>bar</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_29() {
    let input = "Foo\n---\nbar";
    let output = "<h2>Foo</h2><p>bar</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_30() {
    let input = "* Foo\n* * *\n* Bar";
    let output = "<ul><li>Foo</li></ul><hr /><ul><li>Bar</li></ul>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_31() {
//    let input = "- Foo\n- * * *";
//    let output = "<ul><li>Foo</li></ul><li><hr /></li></ul>";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_32() {
    let input = "# foo\n## foo\n### foo\n#### foo\n##### foo\n###### foo";
    let output = "<h1>foo</h1><h2>foo</h2><h3>foo</h3><h4>foo</h4><h5>foo</h5><h6>foo</h6>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_33() {
    let input = "####### foo";
    let output = "<p>####### foo</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_34() {
    let input = "#5 bolt\n\n#hashtag";
    let output = "<p>#5 bolt</p><p>#hashtag</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_35() {
    let input = "\\## foo";
    let output = "<p>## foo</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_36() {
    let input = "# foo *bar* \\*baz\\*";
    let output = "<h1>foo <em>bar</em> *baz*</h1>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_37() {
    let input = "#                  foo                     ";
    let output = "<h1>foo</h1>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_38() {
    let input = " ### foo\n  ## foo\n   # foo";
    let output = "<h3>foo</h3><h2>foo</h2><h1>foo</h1>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_39() {
    let input = "    # foo";
    let output = "<pre><code># foo</code></pre>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_40() {
    let input = "foo\n    # bar";
    let output = "<p>foo\n# bar</p>";
    assert_eq!(top(input), output);
}
 
#[test]
fn test_example_41() {
    let input = "## foo ##\n  ###   bar    ###";
    let output = "<h2>foo</h2><h3>bar</h3>";

    assert_eq!(top(input), output);
}

#[test]
fn test_example_42() {
    let input = "# foo ##################################\n##### foo ##";
    let output = "<h1>foo</h1><h5>foo</h5>";

    assert_eq!(top(input), output);
}

#[test]
fn test_example_43() {
    let input = "### foo ###     ";
    let output = "<h3>foo</h3>";

    assert_eq!(top(input), output);
}

#[test]
fn test_example_47() {
    let input = "****\n## foo\n****";
    let output = "<hr /><h2>foo</h2><hr />";

    assert_eq!(top(input), output);
}

#[test]
fn test_example_48() {
    let input = "Foo bar\n# baz\nBar foo";
    let output = "<p>Foo bar</p><h1>baz</h1><p>Bar foo</p>";

    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_49() {
//    let input = "## \n#\n### ###";
//    let output = "<h2></h2><h1></h1><h3></h3>";
//
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_50() {
    let input = "Foo *bar*\n=========\n\nFoo *bar*\n---------";
    let output = "<h1>Foo <em>bar</em></h1><h2>Foo <em>bar</em></h2>";

    assert_eq!(top(input), output);
}

#[test]
fn test_example_51() {
    let input = "Foo *bar\nbaz*\n====";
    let output = "<h1>Foo <em>bar\nbaz</em></h1>";

    assert_eq!(top(input), output);
}

#[test]
fn test_example_52() {
    let input = "Foo\n-------------------------\n\nFoo\n=";
    let output = "<h2>Foo</h2><h1>Foo</h1>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_53() {
//    let input = "   Foo\n---\n  Foo\n-----\n  Foo\n  ===";
//    let output = "<h2>Foo</h2><h2>Foo</h2><h1>Foo</h1>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_54() {
//    let input = "    Foo\n    ---\n    Foo\n---";
//    let output = "<pre><code>Foo\n---\n\nFoo</code></pre><hr />";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_55() {
//    let input = "Foo\n   ----      ";
//    let output = "<h2>Foo</h2>";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_56() {
    let input = "Foo\n    ---";
    let output = "<p>Foo\n---</p>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_57() {
//    let input = "Foo\n\n= =\nFoo\n--- -";
//    let output = "<p>Foo\n= =</p><p>Foo</p><hr />";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_58() {
    let input = "Foo  \n-----";
    let output = "<h2>Foo</h2>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_59() {
    let input = "Foo\\\n----";
    let output = "<h2>Foo\\</h2>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_60() {
//    let input = "`Foo\n----\n`\n\n<a title=\"a lot\n---\nof dashes\"/>";
//    let output = "<h2>`Foo</h2><p>`</p><h2>&lt;a title=&quot;a lot</h2><p>of dashes&quot;/&gt;</p>";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_61() {
    let input = "> Foo\n---";
    let output = "<blockquote><p>Foo</p></blockquote><hr />";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_62() {
//    let input = "> foo\nbar\n===";
//    let output = "<blockquote><p>foobar===</p></blockquote>";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_63() {
    let input = "- Foo\n---";
    let output = "<ul><li>Foo</li></ul><hr />";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_64() {
    let input = "Foo\nBar\n---";
    let output = "<h2>Foo\nBar</h2>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_65() {
    let input = "---\nFoo\n---\nBar\n---\nBaz";
    let output = "<hr /><h2>Foo</h2><h2>Bar</h2><p>Baz</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_66() {
    let input = "\n====";
    let output = "<p>====</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_67() {
    let input = "---\n---";
    let output = "<hr /><hr />";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_68() {
    let input = "- foo\n-----";
    let output = "<ul><li>foo</li></ul><hr />";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_69() {
    let input = "    foo\n---";
    let output = "<pre><code>foo</code></pre><hr />";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_70() {
    let input = "> foo\n-----";
    let output = "<blockquote><p>foo</p></blockquote><hr />";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_71() {
    let input = "\\> foo\n------";
    let output = "<h2>&gt; foo</h2>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_72() {
    let input = "Foo\n\nbar\n---\nbaz";
    let output = "<p>Foo</p><h2>bar</h2><p>baz</p>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_73() {
//    let input = "Foo\nbar\n---\nbaz";
//    let output = "<p>Foo\nbar</p><hr /><p>baz</p>";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_74() {
    let input = "Foo\nbar\n* * *\nbaz";
    let output = "<p>Foo\nbar</p><hr /><p>baz</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_75() {
    let input = "Foo\nbar\n\\---\nbaz";
    let output = "<p>Foo\nbar\n---\nbaz</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_76() {
    let input = "    a simple\n      indented code block";
    let output = "<pre><code>a simple\n  indented code block</code></pre>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_77() {
    let input = "  - foo\n\n    bar";
    let output = "<ul><li><p>foo</p><p>bar</p></li></ul>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_78() {
    let input = "1.  foo\n\n    - bar";
    let output = "<ol><li><p>foo</p><ul><li>bar</li></ul></li></ol>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_79() {
//    let input = "    <a/>\n    *hi*\n\n    - one";
//    let output = "<pre><code>&lt;a/&gt;\n*hi*\n\n- one</code></pre>";
//    assert_eq!(top(input), output);
//}

//#[test]
//fn test_example_80() {
//    let input = "    chunk1\n\n    chunk2\n  \n \n \n    chunk3";
//    let output = "<pre><code>chunk1\n\nchunk2\n\n\n\nchunk3</code></pre>";
//    assert_eq!(top(input), output);
//}

//#[test]
//fn test_example_81() {
//    let input = "    chunk1\n      \n      chunk2";
//    let output = "<pre><code>chunk1\n  \n  chunk2\n</code></pre>";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_82() {
    let input = "Foo\n    bar";
    let output = "<p>Foo\nbar</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_83() {
    let input = "    foo\nbar";
    let output = "<pre><code>foo</code></pre><p>bar</p>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_84() {
//    let input = "# Heading/n    foo\nHeading\n------\n    foo\n----";
//    let output = "<h1>Heading</h1><pre><code>foo</code></pre><h2>Heading</h2><pre><code>foo</code></pre><hr />";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_85() {
    let input = "        foo\n    bar";
    let output = "<pre><code>    foo\nbar</code></pre>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_86() {
    let input = "\n    \n    foo\n    ";
    let output = "<pre><code>foo</code></pre>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_87() {
    let input = "    foo  ";
    let output = "<pre><code>foo  </code></pre>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_88() {
//    let input = "```\n<\n >\n```";
//    let output = "<pre><code>&lt; &gt;</code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_89() {
//    let input = "```\n<\n >\n```";
//    let output = "<pre><code>&lt; &gt;</code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_90() {
//    let input = "``\nfoo\n``";
//    let output = "<p><code>foo</code></p>";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_91() {
    let input = "```\naaa\n~~~\n```";
    let output = "<pre><code>aaa\n~~~\n</code></pre>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_92() {
    let input = "~~~\naaa\n```\n~~~";
    let output = "<pre><code>aaa\n```\n</code></pre>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_93() {
//    let input = "````\naaa\n```\n``````";
//    let output = "<pre><code>aaa```</code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_94() {
//    let input = "~~~~\naaa\n~~~\n~~~~";
//    let output = "<pre><code>aaa\n~~~</code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_95() {
//    let input = "```";
//    let output = "<pre><code></code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_96() {
//    let input = "`````\n\n```\naaa";
//    let output = "<pre><code>```\naaa</code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_97() {
//    let input = "> ```\n> aaa\n\nbbb";
//    let output = "<blockquote><pre><code>aaa</code></pre></blockquote><p>bbb</p>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_98() {
//    let input = "```\n\n  \n```";
//    let output = "<pre><code>\n  </code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_99() {
//    let input = "```\n```";
//    let output = "<pre><code></code></pre>";
//    assert_eq!(top(input), output);
//}

//#[test]
//fn test_example_100() {
//    let input = " ```\n aaa\naaa\n```";
//    let output = "<pre><code>aaa\naaa</code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_101() {
//    let input = "  ```\naaa\n  aaa\naaa\n  ```";
//    let output = "<pre><code>aaa\naaa\naaa</code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_102() {
//    let input = "   ```\n   aaa\n    aaa\n  aaa\n   ```";
//    let output = "<pre><code>aaa\n aaa\naaa</code></pre>";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_103() {
    let input = "    ```\n    aaa\n    ```";
    let output = "<pre><code>```\naaa\n```</code></pre>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_104() {
//    let input = "```\naaa\n  ```";
//    let output = "<pre><code>aaa</code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_105() {
//    let input = "   ```\naaa\n  ```";
//    let output = "<pre><code>aaa</code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_106() {
//    let input = "```\naaa\n    ```";
//    let output = "<pre><code>aaa\n    ```</code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_107() {
//    let input = "``` ```\naaa";
//    let output = "<p><code></code>aaa</p>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_108() {
//    let input = "~~~~~~\naaa\n~~~ ~~";
//    let output = "<pre><code>aaa\n~~~ ~~</code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_109() {
//    let input = "foo\n```\nbar\n```\nbaz";
//    let output = "<p>foo</p><pre><code>bar</code></pre><p>baz</p>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_110() {
//    let input = "foo\n---\n~~~\nbar\n~~~\n# baz";
//    let output = "<h2>foo</h2><pre><code>bar</code></pre><h1>baz</h1>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_111() {
//    let input = "```ruby\ndef foo(x)\n  return 3\nend\n```";
//    let output = "<pre><code class=\"language-ruby\">def foo(x)/n  return 3\nend</code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_112() {
//    let input = "~~~~    ruby startline=3 $%@#$\ndef foo(x)\n  return 3\nend\n~~~~~~~";
//    let output = "<pre><code class=\"language-ruby\">def foo(x)\n  return 3\nend</code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_113() {
//    let input = "````;\n````";
//    let output = "<pre><code class=\"language-;\"></code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_114() {
//    let input = "``` aa ```\nfoo";
//    let output = "<p><code>aa</code>foo</p>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_115() {
//    let input = "```\n``` aaa\n```";
//    let output = "<pre><code>``` aaa</code></pre>";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_160() {
    let input = "   [foo]: \n      /url  \n           'the title'  \n\n[foo]";
    let output = "<p><a href=\"/url\" title=\"the title\">foo</a></p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_182() {
    let input = "aaa\n\nbbb";
    let output = "<p>aaa</p><p>bbb</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_183() {
    let input = "aaa\nbbb\n\nccc\nddd";
    let output = "<p>aaa\nbbb</p><p>ccc\nddd</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_184() {
    let input = "aaa\n\n\nbbb";
    let output = "<p>aaa</p><p>bbb</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_185() {
    let input = "  aaa\n bbb";
    let output = "<p>aaa\nbbb</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_186() {
    let input = "aaa\n             bbb\n                                       ccc";
    let output = "<p>aaa\nbbb\nccc</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_187() {
    let input = "   aaa\nbbb";
    let output = "<p>aaa\nbbb</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_188() {
    let input = "    aaa\nbbb";
    let output = "<pre><code>aaa</code></pre><p>bbb</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_189() {
    let input = "aaa     \nbbb     ";
    let output = "<p>aaa<br />bbb</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_190() {
    let input = "  \n\naaa\n  \n\n# aaa\n\n  ";
    let output = "<p>aaa</p><h1>aaa</h1>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_199() {
    let input = "> # Foo\n> bar\n> baz";
    let output = "<blockquote><h1>Foo</h1><p>bar\nbaz</p></blockquote>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_200() {
    let input = "># Foo\n>bar\n> baz";
    let output = "<blockquote><h1>Foo</h1><p>bar\nbaz</p></blockquote>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_201() {
    let input = "   > # Foo\n   > bar\n > baz";
    let output = "<blockquote><h1>Foo</h1><p>bar\nbaz</p></blockquote>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_202() {
//    let input = "    > # Foo\n    > bar\n    baz";
//    let output = "<blockquote><h1>Foo</h1><p>bar\nbaz</p></blockquote>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_203() {
//    let input = "> # Foo\n> bar\nbaz";
//    let output = "<blockquote><h1>Foo</h1><p>barbaz</p></blockquote>";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_204() {
    let input = "> bar\nbaz\n> foo";
    let output = "<blockquote><p>bar\nbaz\nfoo</p></blockquote>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_205() {
    let input = "> foo\n---";
    let output = "<blockquote><p>foo</p></blockquote><hr />";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_206() {
    let input = "> - foo\n- bar";
    let output = "<blockquote><ul><li>foo</li></ul></blockquote><ul><li>bar</li></ul>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_207() {
    let input = ">     foo\n    bar";
    let output = "<blockquote><pre><code>foo</code></pre></blockquote><pre><code>bar</code></pre>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_208() {
//    let input = "> ```\nfoo\n```";
//    let output = "<blockquote><pre><code></code></pre></blockquote><p>foo</p><pre><code></code></pre>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_209() {
//    let input = "> foo\n    - bar";
//    let output = "<blockquote><p>foo- bar</p></blockquote>";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_210() {
    let input = ">";
    let output = "<blockquote></blockquote>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_211() {
    let input = ">\n>  \n> ";
    let output = "<blockquote></blockquote>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_212() {
    let input = ">\n> foo\n>  ";
    let output = "<blockquote><p>foo</p></blockquote>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_213() {
    let input = "> foo\n\n> bar";
    let output = "<blockquote><p>foo</p></blockquote><blockquote><p>bar</p></blockquote>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_214() {
    let input = "> foo\n> bar";
    let output = "<blockquote><p>foo\nbar</p></blockquote>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_215() {
    let input = "> foo\n>\n> bar";
    let output = "<blockquote><p>foo</p><p>bar</p></blockquote>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_216() {
    let input = "foo\n> bar";
    let output = "<p>foo</p><blockquote><p>bar</p></blockquote>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_217() {
    let input = "> aaa\n***\n> bbb";
    let output = "<blockquote><p>aaa</p></blockquote><hr /><blockquote><p>bbb</p></blockquote>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_218() {
    let input = "> bar\nbaz";
    let output = "<blockquote><p>bar\nbaz</p></blockquote>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_219() {
    let input = "> bar\n\nbaz";
    let output = "<blockquote><p>bar</p></blockquote><p>baz</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_220() {
    let input = "> bar\n>\nbaz";
    let output = "<blockquote><p>bar</p></blockquote><p>baz</p>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_221() {
//    let input = "> > > foo\nbar";
//    let output = "<blockquote><blockquote><blockquote><p>foobar</p></blockquote></blockquote></blockquote>";
//    assert_eq!(top(input), output);
//}

//#[test]
//fn test_example_222() {
//    let input = ">>> foo\n> bar\n>>baz";
//    let output = "<blockquote><blockquote><blockquote><p>foobarbaz</p></blockquote></blockquote></blockquote>";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_223() {
    let input = ">     code\n\n>    not code";
    let output = "<blockquote><pre><code>code</code></pre></blockquote><blockquote><p>not code</p></blockquote>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_224() {
    let input = "a paragraph\nwith two lines.\n\n    indented code\n\n> A block quote.";
    let output = "<p>a paragraph\nwith two lines.</p><pre><code>indented code</code></pre><blockquote><p>A block quote.</p></blockquote>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_225() {
    let input =
        "1.  a paragraph\n    with two lines.\n\n        indented code\n\n    > A block quote.";
    let output = "<ol><li><p>a paragraph\nwith two lines.</p><pre><code>indented code</code></pre><blockquote><p>A block quote.</p></blockquote></li></ol>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_226() {
    let input = "- one\n\n two\n";
    let output = "<ul><li>one</li></ul><p>two</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_227() {
    let input = "- one\n\n  two\n";
    let output = "<ul><li><p>one</p><p>two</p></li></ul>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_228() {
    let input = " -    one\n\n     two\n";
    let output = "<ul><li>one</li></ul><pre><code> two</code></pre>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_229() {
    let input = " -    one\n\n      two";
    let output = "<ul><li><p>one</p><p>two</p></li></ul>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_274() {
//    let input = "- foo\n- bar\n+ baz";
//    let output = "<ul><li>foo</li><li>bar</li></ul><ul><li>baz</li></ul>";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_301() {
    let input = "\\!\\\"\\#\\$\\%\\&\\'\\(\\)\\*\\+\\,\\-\\.\\/\\:\\;\\<\\=\\>\\?\\@\\[\\\\\\]\\^\\_\\`\\{\\|\\}\\~";
    let output = "<p>!&quot;#$%&amp;'()*+,-./:;&lt;=&gt;?@[\\]^_`{|}~</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_302() {
    let input = "\\→\\A\\a\\ \\3\\φ\\«";
    let output = "<p>\\→\\A\\a\\ \\3\\φ\\«</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_314() {
    let input = "&nbsp; &amp; &copy; &AElig; &Dcaron;\n&frac34; &HilbertSpace; &DifferentialD;\n&ClockwiseContourIntegral; &ngE;";
    let output = "<p>  & © Æ Ď\n¾ ℋ ⅆ\n∲ ≧̸</p>"; // TODO &amp?
    assert_eq!(top(input), output);
}

#[test]
fn test_example_343() {
    let input = "*foo bar*";
    let output = "<p><em>foo bar</em></p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_344() {
    let input = "a * foo bar*";
    let output = "<p>a * foo bar*</p>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_346() {
//    let input = "* a *";
//    let output = "<p>* a *</p>";        ->  should be <ul><li>a *</li></ul>  ?
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_347() {
    let input = "foo*bar*";
    let output = "<p>foo<em>bar</em></p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_348() {
    let input = "5*6*78";
    let output = "<p>5<em>6</em>78</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_632() {
    let input = "foo  \nbaz";
    let output = "<p>foo<br />baz</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_633() {
    let input = "foo\\\nbaz";
    let output = "<p>foo<br />baz</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_634() {
    let input = "foo       \nbaz";
    let output = "<p>foo<br />baz</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_635() {
    let input = "foo  \n     bar";
    let output = "<p>foo<br />bar</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_636() {
    let input = "foo\\\n     bar";
    let output = "<p>foo<br />bar</p>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_637() {
//    let input = "*foo  \nbar*";
//    let output = "<p><em>foo<br />bar</em></p>";
//    assert_eq!(top(input), output);
//}
//
//#[test]
//fn test_example_638() {
//    let input = "*foo\\\nbar*";
//    let output = "<p><em>foo<br />bar</em></p>";
//    assert_eq!(top(input), output);
//}

#[test]
fn test_example_643() {
    let input = "foo\\";
    let output = "<p>foo\\</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_644() {
    let input = "foo  ";
    let output = "<p>foo</p>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_645() {
    let input = "### foo\\";
    let output = "<h3>foo\\</h3>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_646() {
    let input = "### foo  ";
    let output = "<h3>foo</h3>";
    assert_eq!(top(input), output);
}

#[test]
fn test_example_647() {
    let input = "foo\nbaz";
    let output = "<p>foo\nbaz</p>";
    assert_eq!(top(input), output);
}

//#[test]
//fn test_example_648() {
//    let input = "foo \n baz";
//    let output = "<p>foo\nbaz</p>";
//    assert_eq!(exec(input), output);
//}
