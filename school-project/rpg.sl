import "std/stdio.h"
import "std/stdlib.h"

enum EntityType : int
    Player = 1
    Goblin = 2
end

struct Entity
    entity_type: EntityType
    health: int
    defence: int
    strength: int

    constructor: func(EntityType, int, int, int): Entity*
    damage: func(Entity*, int)
    attack: func(Entity*, Entity*)
end

func Entity.damage(self: Entity*, damage: int)
    self.health = self.health - (damage - self.defence / 10)
end

func Entity.attack(self: Entity*, target: Entity*)
    var damage: int = self.strength
    target.damage(target, damage)
end

func Entity.constructor(entity_type: EntityType, health: int, defence: int, strength: int): Entity*
    var self: Entity* = malloc(sizeof Entity*)
    self.entity_type = entity_type
    self.health = health
    self.defence = defence
    self.strength = strength
    self.constructor = Entity.constructor
    self.damage = Entity.damage
    self.attack = Entity.attack
    return self
end

func main()
    var player: Entity* = new Entity(EntityType.Player, 100, 10, 10)
    var goblin: Entity* = new Entity(EntityType.Goblin, 20, 5, 5)
end