(defprotocol IBar
  (make-drink [this drink]))

(deftype StandardBar []
  IBar
  (make-drink [this drink]
    (print "Making drink " drink)
    :ok))

(deftype ProxiedBar [db ibar]
  IBar
  (make-drink [this drink]
    (make-drink ibar drink)
    (substract-ingredients db drink)))

(make-drink (StandardBar.)
  {:name "Manhattan"
   :ingredients [["Bourbon" 75] ["Sweet Vermouth" 25] ["Angostura" 5]]})

(make-drink (ProxiedBar. {:db 1} (StandardBar.))
  {:name "Manhattan"
   :ingredients [["Bourbon" 75] ["Sweet Vermouth" 25] ["Angostura" 5]]})
