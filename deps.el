(package-archive-contents)

;; 获取 melpa 上的 archive-contents
(url-copy-file "https://elpa.emacs-china.org/melpa/archive-contents" "h:/ElectronProject/matrix/222.el" t)

(package-read-all-archive-contents)

;; Emacs buildin mode package [[https://wikemacs.org/wiki/Package.el][Package.el]]
(require 'package)
