# Erstellung einer Wissensdatenbank, basierend auf einem semantischen Netz

IPSO Zürich – OOP Grundlagen<br>
bei Christian Herren<br>
25\. Juli 2025<br>
Michael Crepaldi

---

## Management Summary

### Ausgangslage:

Der strukturierte Umgang mit Wissen wird für Unternehmen immer wichtiger auch im Hinblick auf Normen wie ISO 9001. Oft
ist Wissen jedoch unstrukturiert verteilt oder nur schwer zugänglich. Ziel ist daher, einen Prototyp zu entwickeln, der
als basis für eine Einführung eines Solchen systems legen soll.

### Problem:

Wissensverlust durch Fluktuation, doppelte Arbeit und lange Suchzeiten sind häufige Folgen fehlender Struktur.
Klassische Datenbanken reichen nicht aus, um komplexe Zusammenhänge und Begriffe sinnvoll abzubilden.

### Auftrag und Ziele:

Ziel ist die Entwicklung eines Prototyps für eine semantisch aufgebaute Wissensdatenbank. Zentrale Anforderungen sind
der Aufbau einer Datenstruktur in Form eines allgemeinen Baums, das Ablegen der vorgesehenen Wissensstruktur sowie die
Möglichkeit, Wissen gezielt von einem gewählten Knoten aus abzufragen.

Optional sollen eine grafische Darstellung des Wissensnetzes sowie zwei Suchmodi integriert werden: eine einfache,
schnelle Suche und eine vertiefte Variante, die grössere Teile des Netzes einbezieht.

### Vorgehen:

Nach Analyse der Anforderungen wurde ein Zeitplan erstellt, der in mehreren Phasen die gestellten Ziele erfüllen soll.
Aus den Anforderungen wurden realistische Use-Cases definiert und daraus plausible Test-Cases abgeleitet. Anschließend
wurde die Funktionalität implementiert, anhand der Test-Cases überprüft und die Resultate festgehalten.

### Ergebnisse:

Ein einfacher Prototyp wurde erfolgreich erstellt. Er erlaubt die Erstellung und Abfrage von Wissenseinheiten in einem
einfachen semantischen Netz. Die Anwendung ist erweiterbar und performant.

### Empfehlung:

Die Implementierung ist für eine Weiterentwicklung geeignet. Besonders sinnvoll wäre die Integration in bestehende
Wissensmanagementsysteme oder Intranets. Auch ein Web Interface wäre denkbar.

### Weiteres Vorgehen:

- Grafische Visualisierung
- Langfristige Persistenz
- Authentifizierung und Berechtigungen
- Performance Verbesserungen

---

## Inhaltsverzeichnis

1. [Management Summary](#management-summary)
2. [Inhaltsverzeichnis](#inhaltsverzeichnis)
3. [Einleitung](#einleitung)
4. [Projektmanagement](#projektmanagement)
    1. [Planung](#planung)
    2. [Controlling](#controlling)
5. [Analyse und Design](#analyse-und-design)
    1. [Anwendungsfälle](#anwendungsfälle)
    2. [Testfälle](#testfälle)
    3. [Interpretation Datenmodell](#interpretation-datenmodell)
6. [Implementation](#implementation)
7. [Testdurchführung und Auswertung](#testdurchführung-und-auswertung)
    1. [Bewertung](#bewertung)
    2. [Fazit](#fazit)
8. [Rückblick](#rückblick)
    1. [Zeitplanauswertung](#zeitplanauswertung)
    2. [Fazit Controlling](#fazit-controlling)
9. [Fazit und Ausblick](#fazit-und-ausblick)
10. [KI Deklaration](#ki-deklaration)
11. [Eigenständigkeitserklärung](#eigenständigkeitserklärung)

---

## Einleitung

Wissen ist eine zentrale Ressource moderner Unternehmen. Um es strukturiert zu erfassen, zu pflegen und zugänglich zu
machen, wurde im Rahmen dieses Projekts ein Prototyp entwickelt, der auf einem semantischen Netz basiert. Die
Implementierung erfolgte vollständig in der Programmiersprache Rust, da diese durch ihre Sicherheit, Performance und
moderne Toolchain überzeugt.

---

## Projektmanagement

### Planung

Der Projektablauf wurde in klar definierte Phasen unterteilt, wobei jede Phase konkrete Aufgaben und eine Zeitvorgabe
erhielt. Die Arbeitspakete bauten logisch aufeinander auf und ermöglichten eine strukturierte und schrittweise
Umsetzung. Der Zeitplan diente dabei als Orientierung für die jeweiligen Projektschritte.

#### Zeitplan

| Phase                   | Aufgaben                                                                                          | Dauer | Startdatum       | Enddatum         |
|-------------------------|---------------------------------------------------------------------------------------------------|-------|------------------|------------------|
| 1. Konzeption           | - Anforderungsanalyse<br/>- Datenmodell<br/>- Architekturkonzept<br/>- Use Cases<br/>- Test Cases | 3.5 h | 24.07.2025 15:00 | 24.07.2025 18:30 |
| 2. Basis Implementation | - Datenstruktur<br/>- Befüllen der Struktur<br/>- Abfrage der Struktur<br/>- Persistierung        | 2.5 h | 24.07.2025 18:30 | 24.07.2025 21:00 |
| 3. Erweiterungen        | - Einfache Suche<br/>- Erweiterte Suche<br/>- Konzept grafische Darstellung                       | 2 h   | 24.07.2025 21:00 | 24.07.2025 23:00 |
| 4. Testing              | - Testcases Implementieren und durchführen                                                        | 2 h   | 25.07.2025 11:00 | 25.07.2025 13:00 |
| 5. Dokumentation        | - Fertigstellung der Projektdokumentation                                                         | 5 h   | 25.07.2025 13:00 | 25.07.2025 18:00 |

### Controlling

Das Projektcontrolling wird kontinuierlich durch folgende methoden Erfolgen:

- Zeit Tracking mit Aufzeichnung
- Fortschrittskontrollen nach jeder Phase
- Reflexion und Anpassung des weiteren Vorgehens bei Hindernissen

---

## Analyse und Design

### Anwendungsfälle

| Nr | Use Case                 | Beschreibung                                                       | Testziele |
|----|--------------------------|--------------------------------------------------------------------|-----------|
| 1  | Knoten erstellen         | Ein Knoten mit einer Bedeutung kann erstellt werden                | 1         |
| 2  | Kante erstellen          | Verbindung zwischen zwei Knoten mit Bedeutung kann erstellt werden | 2         |
| 3  | Knoten finden            | Knoten kann über sein Label gefunden werden                        | 3         |
| 4  | Einfach Suche            | Ausgehende Verbindungen eines Knotens können angezeigt werden      | 4         |
| 5  | Erweiterte Suche         | Nachbar Knoten können abgefragt werden                             | 5         |
| 6  | Struktur speichern/laden | Wissensstruktur kann serialisiert und deserialisiert werden        | 6, 7      |
| 7  | Persistenz in Datei      | Wissensstruktur kann als Datei gespeichert/gelesen werden          | 8, 9      |

### Testfälle

Für jeden Anwendungsfall und für die zentralen Komponenten des Datenmodells werden Testfälle definiert. Hier einige
Beispiele:

| Nr. | Test Case                       | Aktion                                                         | Erwartetes Ergebnis                                       |
|-----|---------------------------------|----------------------------------------------------------------|-----------------------------------------------------------|
| 1   | Knoten erstellen                | Erstelle einen neuen Knoten mit Label "Katze"                  | Knoten "Katze" wird erfolgreich erstellt                  |
| 2   | Kante erstellen                 | Erstelle eine Kante von "Katze" zu "Maus" mit Bedeutung "jagt" | Kante "jagt" zwischen Knoten "Katze" und  "Maus" erstellt |
| 3   | Knoten finden                   | Suche nach dem Knoten "Maus"                                   | Knoten "Maus" wird gefunden                               |
| 4   | Einfache Suche                  | Frage die direkten Verbindungen von "Katze" ab                 | Verbindungen wie "Katze jagt" werden angezeigt            |
| 5   | Erweiterte Suche                | Frage alle benachbarten Knoten von "Katze" ab                  | Verbindungen wie "Katze jagt Maus" werden angezeigt       |
| 6   | Graph serialisieren             | Wandle Graph in Text um                                        | Textliche Darstellung des Graphen                         |
| 7   | Graph aus Text wiederherstellen | Wandle den Text in einen Graphen um                            | Ursprüngliche Struktur wird korrekt wiederhergestellt     |
| 8   | Graph speichern                 | Speichere den Graph als Datei                                  | Datei enthält gültige Darstellung                         |
| 9   | Graph laden                     | Lade den Graph aus Datei                                       | Graph wird vollständig und korrekt geladen                |

### Interpretation Datenmodell

Das folgende Datenmodell bildet die Grundlage für ein einfaches semantisches Netzwerk in Rust. Es orientiert sich am
Klassendiagramm, wurde aber unter Berücksichtigung der Besonderheiten der Programmiersprache Rust angepasst und
erweitert.

```rust
struct Node {
    label: String,
    links: Vec<Rc<RefCell<Link>>>,
}

struct Link {
    label: String,
    node: Rc<RefCell<Node>>,
}

struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>,
}
```

Obwohl im deutschsprachigen Raum üblicherweise Begriffe wie `Knoten` oder `Kante` verwendet werden, wurde hier bewusst
auf englischsprachige Begriffe wie `Node` und `Link` zurückgegriffen, da sie in der Softwareentwicklung gebräuchlicher
sind.

Die Struktur wurde um das `Graph` Objekt erweitert, das als Einstiegspunkt für das Netzwerk dient. Es erleichtert den
Zugriff und die Verwaltung aller enthaltenen Knoten.

**Node** - eine Node stellt ein einzelnes Wissenselement oder Konzept dar (z.B. "Katze", "Tier" oder "Maus"). Jeder
Knoten
besitzt:

- ein eindeutiges label, das zur Beschreibung des Konzepts dient
- eine Liste ausgehender Verbindungen (links), die auf andere Knoten zeigen

**Link** - ein Link beschreibt eine semantische Beziehung zwischen zwei Knoten. Ein Link besteht aus:

- einem label das die Beziehung angibt (z.B. ist-ein, jagt-eine)
- eine Verbindung zum nächsten Knoten

**Graph** - der Graph dient als Container für alle Knoten im Netzwerk. Er abstrahiert das semantische Netz als Ganzes
und ermöglicht:

- eine zentrale Verwaltung aller Wissenseinheiten
- einen einfachen Zugriff auf alle Knoten

**Technische Umsetzung:** Rc (Reference Counting) erlaubt es, dass mehrere Besitzer eine Instanz teilen und automatisch
verwalten, wie lange sie existiert. RefCell sorgt dafür, dass diese Instanz auch dann noch veränderbar bleibt, wenn
mehrere Besitzer darauf zugreifen, obwohl Rust dies normalerweise nicht erlaubt. Das ist ein bewusster Workaround, um
die strengen Regeln von Rust bei Ownership und Mutability vorübergehend zu umgehen und etwas Flexibilität
zurückzugewinnen.

---

## Implementation

Die Umsetzung des semantischen Netzwerks basiert auf einer klaren Trennung zwischen den zentralen Strukturen `Graph`,
`Node` und `Link`. Der `Graph` koordiniert die Verwaltung der Knoten und Relationen, während `Node` und `Link` die
eigentlichen semantischen Einheiten und Verbindungen abbilden.

Die Struktur `Node` stellt einzelne Konzepte dar. Über die Methode `Node::new()` wird ein neuer Knoten mit
einem label erzeugt. Beziehungen zu anderen Knoten können über `add_link()` hinzugefügt werden. Diese Methode erzeugt
intern eine neue `Link` Instanz und fügt sie der internen `links` Liste hinzu. Dadurch entsteht eine gerichtete und
beschriftete Verbindung von diesem Knoten zu einem anderen. Die Methoden `basic_search()` und `advanced_search()` dienen
dazu die beiden suchmodi durchzuführen. Mit dem einfachen werden die beziehungen von einem Knoten ausgelesen während mit
der erweiterten Suche auch die verknüpften Knoten abgerufen werden können

Die Struktur `Link` dient als Abbildung der semantischen Beziehung selbst. Sie wird mit `Link::new()` erzeugt und
enthält neben der Relation (z.B. ist-ein) auch eine Referenz auf das Zielobjekt der Beziehung.

Der `Graph` selbst bietet Methoden zur Erstellung `Graph::new()`, Erweiterung `add_node()`, `link()` und Suche
`get_node()` innerhalb des Netzwerks wobei diese nicht mit den Suchoperationen verwechselt werden darf. Die Funktion
`link()` stellt sicher, dass nur existierende Knoten verbunden werden, und kapselt den Zugriff auf
`get_or_create_node()` und endgültig dann `add_link()` der Ursprungs Node.

Zusätzlich wird ein einfaches Parsen und Serialisieren zur Persistierung ermöglicht: Durch die Implementierung des
Rust Traits `Into` sowohl für `String` als auch für `Graph` kann zwischen Text und Netzwerkstruktur konvertiert werden.
Die Implementierung `impl Into<Graph> for String` erlaubt es, einen Graphen aus textuellen Aussagen zu erzeugen. Der
Parser erwartet dabei eine sehr einfache Satzstruktur im Format:
`<füllwort> <subjekt> <relation-1> <relation-2> <objekt>` (z.B. eine katze ist ein tier) und leitet daraus die
entsprechenden Knoten und Links ab.

Umgekehrt ermöglicht `impl Into<String> for Graph` die Serialisierung des Netzwerks in eine textuelle Form. Dabei wird
jede gespeicherte semantische Beziehung in einen umgewandelt. Diese Konvertierungen nutzen das Trait System von Rust zur
Definition benutzerdefinierter Transformationen zwischen Datentypen und ermöglichen so eine einfache Persistierung oder
Weitergabe des Netzwerks in textueller Form.

---

## Testdurchführung und Auswertung

Für jeden Testfall in der folgenden Tabelle wurde ein entsprechender Unit-Test implementiert. Diese Tests decken die
zentralen Anforderungen ab und stellen sicher, dass die Kernfunktionen wie erwartet arbeiten. Die Tests sind nicht
abschliessend, bieten aber eine solide Grundlage zur Überprüfung der wichtigsten Funktionalitäten des semantischen
Netzwerks.

Die folgende Tabelle dokumentiert die Ergebnisse der Testausführung:

| Test Case | Erwartetes Ergebnis                                      | Tatsächliches Ergebnis                                      | Bewertung             |
|-----------|----------------------------------------------------------|-------------------------------------------------------------|-----------------------|
| 1         | Knoten "Katze" wird erfolgreich erstellt                 | Knoten "Katze" erfolgreich erstellt und im Graph enthalten  | Erfüllt               |
| 2         | Kante "jagt" zwischen Knoten "Katze" und "Maus" erstellt | Kante "jagt" in `katze.links` vorhanden, Ziel: "maus"       | Erfüllt               |
| 3         | Knoten "Maus" wird gefunden                              | `get_node("maus")` liefert gültiges Objekt                  | Erfüllt               |
| 4         | Verbindungen wie "Katze jagt" werden angezeigt           | Nur direkte Verbindungen (`links`) werden korrekt angezeigt | Erfüllt               |
| 5         | Verbindungen wie "Katze jagt Maus" werden angezeigt      | Ausgabe von "katze jagt maus" wird korrekt gemacht          | Erfüllt               |
| 6         | Textliche Darstellung des Graphen                        | String Repräsentation enthält alle Daten                    | Erfüllt               |
| 7         | Ursprüngliche Struktur wird korrekt wiederhergestellt    | Deserialisierung erzeugt dieselbe Struktur                  | Erfüllt               |
| 8         | Datei enthält gültige Darstellung                        | Datei mit korrektem Inhalt wird erzeugt                     | Nicht erfüllt (Minor) |
| 9         | Graph wird vollständig und korrekt geladen               | Struktur des geladenen Graphen stimmt mit Original überein  | Nicht erfüllt (Minor) |

### Fazit

Die zentralen Operationen zur Erstellung, Verknüpfung und Abfrage von Knoten funktionieren stabil und entsprechen den
Anforderungen. Verbesserungsbedarf besteht insbesondere bei der Erweiterung um rekursive Abfragen und echte
Datei Operationen zur langfristigen Persistierung und Wiederverwendung von Graphen.

---

## Rückblick

Die Projektumsetzung verlief insgesamt planmässig und ohne grössere Verzögerungen. Die einzelnen Phasen konnten in der
vorgesehenen Reihenfolge bearbeitet werden. Kleinere zeitliche Verschiebungen insbesondere bei der Dokumentation und
beim Testing konnten durch effiziente Vorarbeit in der Entwicklungsphase ausgeglichen werden.

Durch kontinuierliches Controlling mittels Zeittracking und regelmässiger Zwischenreflexionen liess sich der
Projektfortschritt realistisch einschätzen. Die folgende Tabelle zeigt die Soll-Ist-Abweichungen je Phase:

### Zeitplanauswertung

| Phase                   | Geplante Dauer | Tatsächliche Dauer | Abweichung  |
|-------------------------|----------------|--------------------|-------------|
| 1. Konzeption           | 3.5 h          | 3.5 h              | -           |
| 2. Basis Implementation | 2.5 h          | 3.0 h              | + 0.5 h     |
| 3. Erweiterungen        | 2 h            | 1.5 h              | − 0.5 h     |
| 4. Testing              | 2 h            | 2.3 h              | + 0.3 h     |
| 5. Dokumentation        | 5 h            | 6 h                | + 1 h       |
| **Gesamt**              | **15.0 h**     | **16.3 h**         | **+ 1.3 h** |

### Fazit Controlling

Für künftige Projekte empfiehlt es sich mehr Zeit für die Dokumentation einzuplanen da dieser Teil oft in seiner
komplexität unterschätzt wird. Auch eine etwas grosszügigere Planung für die Testphase könnte sinnvoll sein, um
genügend Puffer für unvorhergesehene Korrekturen zu haben.

---

## Fazit und Ausblick

Die Arbeit zeigt, wie ein einfaches semantisches Netzwerk in Rust effizient modelliert und umgesetzt werden kann. Durch
den Einsatz von `Rc` und `RefCell` konnten dynamische, veränderbare Graph Strukturen erstellt werden, ohne die
Sicherheitsgarantien der Sprache vollständig aufzugeben. Die Umwandlung zwischen Text und Graph bietet zudem einen
ersten Schritt zur Verarbeitung natürlicher Sprache.

Die Basisfunktionen das Erstellen, Verknüpfen und Abfragen von Knoten wurden erfolgreich implementiert und getestet.
Einschränkungen bestehen derzeit bei rekursiven Abfragen und echten Datei Operationen die in einer erweiterten Version
ergänzt werden könnten.

Ein sinnvoller nächster Schritt ist die Umstellung der internen `nodes` Liste im `Graph` auf eine `HashMap`, um einen
direkten O(1) Zugriff auf Knoten über ihr Label zu ermöglichen. Dies verbessert die Effizienz der Suche erheblich und
bereitet das System auf grössere Datenmengen vor.

---

## KI Deklaration

| Werkzeug | Zweck                                        | Einsatzbereich    |
|----------|----------------------------------------------|-------------------|
| ChatGPT  | Grammatik, Satzbau und Rechtschreibkorrektur | Gesamtes Dokument |

---

## Eigenständigkeitserklärung

Ich versichere hiermit, dass ich die vorliegende Arbeit selbstständig verfasst und keine anderen als die angegebenen
Quellen und Hilfsmittel benutzt habe. Alle Stellen, die wörtlich oder sinngemäss aus veröffentlichten oder nicht
veröffentlichten Schriften entnommen wurden, sind als Zitate kenntlich gemacht. Diese Arbeit ist in gleicher oder
ähnlicher Form noch keiner anderen Prüfungsbehörde vorgelegt worden.

Zürich, 25.07.2025<br/>Michael Crepaldi
