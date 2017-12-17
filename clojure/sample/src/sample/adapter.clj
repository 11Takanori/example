{:name "Lancelot"
 :speed 1.0
 :attack-bow-fn attack-with-bow
 :attack-sword-fn attack-with-sword
 :block-fn block-with-shield}

{:name "Commando"
 :speed 5.0
 :attack-bow-fn (partial throw-grenda "F1")
 :attack-sword-fn (partial shot "M16")
 :block-fn nil}
