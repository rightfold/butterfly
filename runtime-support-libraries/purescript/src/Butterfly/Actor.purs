module Butterfly.Actor
  ( Actor (..)
  ) where

import Prelude

newtype Actor =
  Actor String

derive newtype instance eqActor :: Eq Actor
derive newtype instance ordActor :: Ord Actor
