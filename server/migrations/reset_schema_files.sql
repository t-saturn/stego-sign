-- Elimina todos los datos pero conserva las tablas y el schema
TRUNCATE files.objects RESTART IDENTITY CASCADE;

TRUNCATE files.buckets RESTART IDENTITY CASCADE;