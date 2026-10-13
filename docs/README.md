# Documentation

- `[ARCHITECTURE.md](ARCHITECTURE.md)` : décisions métier, diagrammes, description des tables.
- `[sea-orm.md](db/sea-orm.md)` : migrations, commandes et conventions d'écriture.
- `[schema.sql](db/schema.sql)` : copie lisible des 9 tables. Ne pas l'exécuter.
- `[views.sql](db/views.sql)` : copie lisible des 2 vues de périmètre. Ne pas l'exécuter.
- `[testing.md](backend/testing.md)` : organisation des tests du backend.

Le schéma en base est celui de `[migration/](../migration/)`. `schema.sql` et `views.sql` se mettent à jour en même temps qu'une nouvelle migration. Les points ouverts sont dans « Points encore ouverts » de `ARCHITECTURE.md`.
