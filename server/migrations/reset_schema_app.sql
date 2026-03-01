-- Elimina todos los datos pero conserva las tablas y el schema
TRUNCATE app.audit_log RESTART IDENTITY CASCADE;

TRUNCATE app.documents RESTART IDENTITY CASCADE;