(ns application.facade
  (:require [application.request-extractor :as re])
  (:requier [application.request-validator :as rv])
  (:require [application.transformer :as t])
  (:require [application.response-builder :as rb]))

(defn request-extract [request]
  (re/extract request))

(defn request-validate [request]
  (rv/validate request))

(defn request-transform [request]
  (t/transformer request))

(defn response-build [request]
  (rb/build request))



(ns application.old-servlet
  (:use [application.facade]))

(defn service [request]
  (-> request
      (request-extract)
      (request-validate)
      (request-transform)
      (request-build)))
