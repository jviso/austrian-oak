# austrian-oak

Utility for keeping a journal of strength training, aka a lifting log.
[Get it?](https://www.youtube.com/watch?v=D8BUdXraZ1g)

## Set-up

### Installation

There is not a distribution or package of any kind. Compile from source directly.

### Journal file

`austrian-oak` stores events in a simple JSONL file. Before logging an event, this file must exist,
and its path must be described by the environment variable `AUSTRIAN_OAK_JOURNAL_LOCATION`.

### State file

`austrian-oak` stores current state in a second JSONL file. Before using the app, this file must
exist, and its path must be described by the environment variable `AUSTRIAN_OAK_STATE_LOCATION`.

## Usage

### Log an event

Log a single event like so:
```shell
# Log a set on deadlift: 345 pounds for 6 repetitions
$ /path/to/binary log deadlift,345,6,2024-01-01
# Omit the date to default to today
$ /path/to/binary log squat,315,5
# Log a night of sleep
$ /path/to/binary log sleep,good,7h
# Log a day of nutrition
$ /path/to/binary log nutrition,good
```
Needed soon: a command to list all supported event types and their associated data.

### Using shortcodes

Each event has a two-letter shortcode to save some keystrokes:
```shell
# Log a set on deadlift: 345 pounds for 6 repetitions
$ /path/to/binary log dl,345,6,2024-01-01
# Omit the date to default to today
$ /path/to/binary log sq,315,5
# Log a night of sleep
$ /path/to/binary log sl,good,7h
# Log a day of nutrition
$ /path/to/binary log nu,good
```
Needed soon: a command to list all shortcodes.

### Upcoming

- Reporting
  - Details of a given event type on the last _n_ days it appears
  - Details of last _n_ strength training sessions (all sets of all strength events per day)
  - Personal records per strength event
  - Plots of events and correlations over time (e.g., sleep + nutrition, nutrition + mass, etc.)

