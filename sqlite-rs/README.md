# sqlite-rs

Поэтапное создание простой базы данных (клон sqlite с нуля на Rust).

### Как работает база данных?

- В каком формате сохраняются данные? (в памяти или на диске)
- Когда он перехлдит из памяти на диск?
- Почему в каждой таблице может быть только один первичный ключ?
- Как работает откат транзакции?
- Как форматируются индексы?
- Когда и как происходит полное сканированние таблицы?
- В каком формате сохраняется подготовленное заявление?

Короче говоря, как работает **база данных**?

![Архитектура sqlite](/sqlite-rs/arch2.gif "Архитектура sqlite -- https://www.sqlite.org/arch.html")

Я создаю клон **sqlite** с нуля на языке Rust, чтобы понять, и документировать свой процесс по ходу дела.

### Содержание

- Часть 1 - Введение и настройка REPL;
- Часть 2 - Самый простой компилятор SQL и виртуальная машина;

### Ссылки:

1. https://cstack.github.io/db_tutorial/