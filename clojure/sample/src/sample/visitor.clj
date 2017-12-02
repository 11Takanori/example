(defmulti export
  (fn [item format] [(:type item) format]))

(defmethod export [:activity :pdf] [item format]
  (exporter/activity->pdf item))

(defmethod export [:activity :xml] [item format]
  (exporter/activity->xml item))

(defmethod export [:message :pdf] [item format]
  (exporter/message->pdf item))

(defmethod export [:message :xml] [item format]
  (exporter/message->xml item))

(defmethod export :default [item format]
  (throw (IllegalArgumentException. "not supported")))

;; Message
;; {:type :message :content "Say what again!"}
;; Activity
;;{:type :activity :content "Quoting Ezekiel 25:17"}
;; Formats
;;:pdf, :xml
