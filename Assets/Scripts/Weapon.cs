using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

public class Weapon : Cell
{
    new private void Awake() {
        base.Awake();
    }

    new private void FixedUpdate()
    {
        base.FixedUpdate();
        attack();
    }

    void attack()
    {
        if (energy < Settings.inst.weapon.attackCost) {
            return;
        }

        Collider2D[] near = Physics2D.OverlapCircleAll(transform.position, Settings.inst.weapon.attackRadius);
        foreach (var collider in near) {
            if (energy < Settings.inst.weapon.attackCost) {
                return;
            }

            Cell obj = collider.GetComponentInParent<Cell>();
            if (obj == null || obj.organismID == organismID) {
                continue;
            }

            var stolen = obj.energy * Settings.inst.weapon.drainRate;
            energy += stolen;
            obj.energy -= stolen;

            energy -= Settings.inst.weapon.attackCost;
        }
    }
}

