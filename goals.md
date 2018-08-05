# Goals of `actions`
The features of the library that `actions` should become.

## Eliminating side-effects
### Why
- Reduces the risk of bugs
- Code becomes **modular**!
    - An element/struct can be taken out without any issues, no worries about what it affects: **it only affects things it owns**
- Without side-effects it is much easier to **reason about the code** and what it does

### How
1.  Making sure every datastructure can **only access it's children**
    - **Updates flow down**
    - Optional **requests for more data flow back up** in the return statements

2.  Force the programmer to **return the inverse of each action** when an action is 'performed'
    - This is to make sure every action is well-defined
    - **Inverse can be tested!** Generate a random state, execute the action, then the inverse. Assert that the new state equals the initially generated state.


## Enabling the API-consumer to easily integrate *undo* and *redo* functionality
### Why
Undo and redo functionality is an essential requirement for a lot of software. Users heavily rely on it.

### How
1. Letting the API-consumer implement a function **that receives actions and does something expected based on the action**
    - This function must **return an action that sets the state exactly back** to how it was before the action was received
2. Letting the API-consumer **specify which actions should merge directly** if executed sequentially. (Example: Microsoft Word removes a whole word when undo'ing instead of removing single characters)


## Allowing the API-consumer to compress a list of actions
**Compressing**: reducing the size of the list (as much as possible) without losing changes.<br>
**This can be tested by** generating a random state, execute the action, then the inverse. The state should then match the originally generated state.

### Why
> compressing seems more like something you would want to do if you want to keep a difference-log of an application. For example, if you are writing an editor for a game-engine. Whenever the user saves, the current chain could be **compressed an stored to the drive** (where size matters). It could then be used to show the differences between saves to the user (the **minimal single actions required to get to the new state**: "You moved this object", etc.).

### How
1. Enabling the **API-consumer to implement logic for merging** actions
2. Exposing a **function in the API that compresses** a `Chain` of actions (a vector containing actions) into a `Chain` of fewer actions.
