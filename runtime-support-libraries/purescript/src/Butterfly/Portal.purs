module Butterfly.Portal
  ( Portal (..)
  , Button (..)
  , buttonActors
  ) where

import Prelude

import Data.Lens (Lens', lens)
import Data.List (List)
import Data.Set (Set)

import Butterfly.Actor (Actor)

data Portal f =
  Portal (List (Button f))

data Button f =
  Button String
         (Set Actor)
         (f Unit)

buttonActors :: ∀ f. Lens' (Button f) (Set Actor)
buttonActors = lens get set
  where get (Button _ actors _) = actors
        set (Button name _ action) actors = Button name actors action
