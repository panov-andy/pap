# pap


### LSP-sublime

Что бы заработал LSP в sublime:

- поставить package manger - где-то через меню
- поставить пакет LSP
- зарегистрировать новый синтаксис - расширив его от обычно текста - Developer -> Tools -> New Syntax

```yaml
%YAML 1.2
---
# http://www.sublimetext.com/docs/syntax.html
name: Mol View Tree format
file_extensions:
  - view.tree
scope: source.view.tree
contexts:
  main: []
```        

- настроить LSP на работу конкретного синтаксиса + включить debug

```yaml
// Settings in here override those in "LSP/LSP.sublime-settings"
{
	 "clients": {
        "tree-lsp": {
            "enabled": true,
            "command": ["/home/awa/_p43/pap/target/debug/pap"], // Update the PATH
            "selector": "source.view.tree",
            "initializationOptions": {}
        }
    },
    "log_debug": true,
}
```
- вызвать панель LSP с логами