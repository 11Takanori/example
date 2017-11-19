(ns examples.cryptovault
  (:use [examples.io :only [IOFactory make-reader make-writer]])
  (:require [clojure.java.io :as io])
  (:import (java.security Keystore KeyStore$SecretKeyEntry
                          KeyStore$PasswordProtenction)
           (javax.crypto KeyGenerator Cipher CipherOutputStream
                         CipherInputStream)))
(deftype Cryptovault [filename keystore password])

(defprotocol Vault
  (init-vault [vault])
  (vault-output-stream [vault])
  (vault-input-stream [vault]))
