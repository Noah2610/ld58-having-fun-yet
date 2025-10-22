# Changelog
## v1.0.2
Accessibility patch with video settings for visual trippiness.

### Changes
- add separate volume settings for music and sound effects
- add video settings menu
- add fullscreen toggle key (`f`) and video setting option [#9]
- add bloom effect, toggleable in video settings
- add animations intensity video setting [#17]
- add "pixel-perfect" video setting, default on
- add quit bind: `ctrl+q`
- add gamepad bind for pause toggle [#6]
- add toggle mute bind: `m`
- minor UI style changes

### Fixes
- reset game over state for consecutive plays [#1]
- mitigate player animation artifacts [`58ba`]

[#1]: https://github.com/Noah2610/ld58-having-fun-yet/issues/1
[#6]: https://github.com/Noah2610/ld58-having-fun-yet/issues/6
[#9]: https://github.com/Noah2610/ld58-having-fun-yet/issues/9
[#17]: https://github.com/Noah2610/ld58-having-fun-yet/issues/17
[`58ba`]: https://github.com/Noah2610/ld58-having-fun-yet/commit/58badb4e8903650bcd22f4d20cbe534c4b4a9549

## v1.0.1
Overall greatly improved playability and fun-factor by fixing bugs and tweaking values.

### Changes
- make hitboxes circular, allowing for much better ricocheting/knockback angles
- bullet can damage enemies for a longer time while it's moving
- show player health in UI at top of screen during gameplay
- play sound effects for shooting without a bullet and for collecting bullet  
  (sfx files were done pre-deadline, just forgot to play them ingame)
- balance changes:
    - reduce enemy spawn frequency
    - adjust knockback and speed values
    - cap bullet velocity so it doesn't completely yeet away anymore

### Fixes
- fix survival timer resuming after dying and pausing then unpausing
- fix logic in increasing visual animations
- fix player & enemy knockback and bullet ricochet calculations [#3] & [#4]
- reset animations when returning to main menu [#2]

[#2]: https://github.com/Noah2610/ld58-having-fun-yet/issues/2
[#3]: https://github.com/Noah2610/ld58-having-fun-yet/issues/3
[#4]: https://github.com/Noah2610/ld58-having-fun-yet/issues/4

## v1.0.0
Jam release, has bugs and unbalanced gameplay.  
At least somewhat playable IMO.
