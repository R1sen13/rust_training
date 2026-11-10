# CLI Таск трекер (учебный)
## ttrack

Небольшой CLI-трекер задач, написанный на Rust.

Проект создан для изучения:

- синтаксиса Rust;
- структур и enum;
- владения и заимствования;
- модулей;
- работы с `Vec`;
- обработки `Option`;
- CLI-команд с помощью `clap`.

## Возможности

Сейчас приложение поддерживает следующие команды:

```text
add       Добавить задачу
list      Показать список задач
complete  Завершить задачу по ID
remove    Удалить задачу по ID
```
## Запуск
```powershell
cargo install --path .
ttrack add <task-name>
ttrack list
ttrack complete <task-id>
ttrack remove <task-id>
```