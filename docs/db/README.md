# Time Manager — livrables DB

- [`ARCHITECTURE.md`](ARCHITECTURE.md) : décisions métier, 2 diagrammes Mermaid, description des tables et justifications.
- [`schema.sql`](schema.sql) : 9 tables PostgreSQL commentées, clés, CHECK, indexes, exclusion constraints.
- [`views.sql`](views.sql) : 2 vues SQL exécutables pour la hiérarchie et le périmètre managers ; 2 projections KPI à finaliser.

**Statut :** proposition à valider, **pas migration de production prête à lancer**. Lire les points de validation de `ARCHITECTURE.md`. Exécuter `schema.sql` avant `views.sql` sur une base de test.
