#!/bin/sh
set -e

_term() {
  echo "Caught SIGTERM signal!"
  if [ -n "$child" ]; then
    kill -TERM "$child" 2>/dev/null
    wait "$child"
  fi
}

# Хук на SIGTERM
trap _term TERM INT

# Виконати pre_start скрипти якщо є
if [ -d "/etc/my_init.d/pre_start" ]; then
  run-parts /etc/my_init.d/pre_start
fi

# Запуск твого Rust-сервера
echo "Starting Rust server..."
/app/server/packer &

# запам’ятовуємо pid
child=$!

# Чекаємо завершення процесу або сигналу
wait "$child"