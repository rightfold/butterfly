module Butterfly.Portal
  ( Portal (..)
  , Button (..)
  ) where

import Prelude

import Data.List (List)
import Data.Set (Set)

import Butterfly.Actor (Actor)

data Portal f =
  Portal (List (Button f))

data Button f =
  Button String
         (Set Actor)
         (f Unit)
