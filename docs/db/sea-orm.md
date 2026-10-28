# SeaORM — migrations

Le schéma Postgres est défini par la crate `[migration/](../migration/)`. Ce sont les migrations qui font foi pour la base. `[schema.sql](schema.sql)` et `[views.sql](views.sql)` restent la référence lisible du modèle, mais ne s'exécutent jamais sur une base : tout changement passe par une nouvelle migration.

Le backend lit les tables via les entités de [`backend/src/entity/`](../backend/src/entity/), une par table. Les commandes pour lancer Postgres sont dans le [README](../README.md) racine.

## Migrations existantes

L'ordre du `vec!` de `[migration/src/lib.rs](../migration/src/lib.rs)` est l'ordre d'exécution. Il suit les clés étrangères.

| Migration                               | Contenu                                    |
| --------------------------------------- | ------------------------------------------ |
| `m20261013_000001_create_user`          | type `user_role`, table `users`            |
| `m20261013_000002_create_auth_tables`   | `oauth_identities`, `auth_sessions`        |
| `m20261013_000003_create_teams`         | `teams`, `team_members`                    |
| `m20261013_000004_enable_btree_gist`    | extension `btree_gist`                     |
| `m20261013_000005_create_work_policies` | `user_work_policies`, `work_schedules`     |
| `m20261013_000006_create_work_sessions` | `work_sessions`                            |
| `m20261013_000007_create_audit_log`     | `audit_log`                                |
| `m20261013_000008_create_scope_views`   | vues `v_team_hierarchy`, `v_manager_scope` |

## Commandes

Depuis la racine, avec `DATABASE_URL` défini :

```bash
cargo run -p migration -- up                 # apply pending migrations
cargo run -p migration -- status
cargo run -p migration -- down               # revert the last one
cargo run -p migration -- generate <name>
```

Sur Postgres, chaque migration s'exécute dans sa propre transaction : si elle échoue, rien n'est appliqué. `fresh`, `refresh` et `reset` suppriment des données : ils sont réservés à une base locale jetable.

## Une migration partagée ne se modifie plus

`seaql_migrations` ne stocke que le nom du fichier (`version`) et la date d'application. Il n'y a pas de checksum. Conséquences :

- modifier un fichier déjà appliqué ne casse rien visiblement, mais la base de ceux qui l'ont déjà passé ne change pas : les schémas divergent en silence ;
- renommer ou supprimer un fichier déjà appliqué fait échouer `up` (_Migration file of version '…' is missing_).

Une correction est donc une nouvelle migration (`alter_…`, `add_…`). Si ta base locale contient d'anciennes versions (par exemple `m20220101_000001_create_table`), repars d'une base vide avec `cargo run -p migration -- fresh`, ce qui supprime les données locales.

## Ajouter une migration

```bash
cargo run -p migration -- generate add_teams_archived_at
```

Le fichier `migration/src/mYYYYMMDD_HHMMSS_<name>.rs` est créé. Vérifie qu'il est déclaré dans `lib.rs` **après** les migrations existantes. `down` doit défaire `up` dans l'ordre inverse, sans `todo!()`. `DROP TABLE` supprime aussi les index et les clés étrangères de la table. Un type ou une extension se supprime après les tables qui l'utilisent.

Avant une PR, vérifie la réversibilité :

```bash
cargo run -p migration -- down && cargo run -p migration -- up
```

## Noms SQL : `DeriveIden`

`#[derive(DeriveIden)]` ne crée rien en base. Il donne des noms Rust vérifiés à la compilation aux identifiants SQL :

```rust
#[derive(DeriveIden)]
enum Users {
    Table,     // table "users" (enum name in snake_case)
    Id,        // column "id"
    FirstName, // column "first_name"
}
```

Chaque fichier redéclare les enums dont il a besoin, avec seulement les variantes utilisées (souvent `Table` et `Id` pour une clé étrangère). C'est voulu : une migration doit rester figée même si un autre fichier ou le modèle Rust du backend change. `Alias::new("users")` produit le même SQL.

Un enum Postgres (`user_role`) est un vrai type en base, créé une seule fois avec `Type::create().as_enum(...)` dans la migration qui l'introduit.

## Conventions d'écriture

- Clé primaire : `pk_uuid(T::Id).default(Expr::cust("gen_random_uuid()"))`. `pk_auto` produit un entier, pas un `uuid`.
- Types : `text`/`text_null`, `uuid`/`uuid_null`, `date`/`date_null`, `time_null`, `small_integer`, `integer_null`, `json_binary_null` (`jsonb`), `timestamp_with_time_zone_default_now`. Les helpers sans `_null` posent `NOT NULL`.
- `string_len` / `string_len_null` produisent `varchar(n)`. Réservé à `users` : noms 100, e-mail 254, téléphone 20, hash 255. Les autres textes restent `text`.
- Enum Postgres : `Type::create().as_enum(...)`, puis `enumeration(colonne, type, valeurs)`. `down` supprime le type après les tables. Types existants : `user_role`, `work_policy_mode`.
- `CHECK` sur une colonne : `.check(Expr::cust("..."))`. Avec un nom : `.check(("nom", Expr::cust("...")))`. Un `CHECK` qui compare plusieurs colonnes se pose sur `Table::create()`.
- Clé étrangère : `.foreign_key(ForeignKey::create().name("<table>_<col>_fkey")...)`, le nom que Postgres donnerait par défaut à un `REFERENCES` de `schema.sql`.
- Postgres n'indexe pas une clé étrangère tout seul : ajoute un `manager.create_index(...)`. `Table::create().index(...)` est réservé à MySQL.
- Index sur expression : `.col(Func::lower(Expr::col(Users::Email)))`. Ordre décroissant : `.col((T::CreatedAt, IndexOrder::Desc))`. Index partiel : `.and_where(Expr::col(T::EndTime).is_null())`.

## SQL brut

Le builder ne sait pas exprimer les contraintes `EXCLUDE`, les extensions ni les vues. On passe alors par :

```rust
manager
    .get_connection()
    .execute_unprepared("CREATE EXTENSION IF NOT EXISTS btree_gist")
    .await?;
```

Exemples : `policies_no_overlap` dans `000005`, `work_sessions_no_overlap` dans `000006`, les vues dans `000008`.

## Vérifier

```bash
docker compose exec -T postgres psql -U brigitte -d mybrigittedb -c '\d'
docker compose exec -T postgres psql -U brigitte -d mybrigittedb -c '\d+ work_sessions'
```
