using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

public class Weapon : Cell
{
    SpriteRenderer icon;
    LineRenderer line;

    new private void Awake() {
        base.Awake();
        icon = transform.Find("Icon").GetComponent<SpriteRenderer>();
        line = GetComponent<LineRenderer>();
    }

    new private void FixedUpdate()
    {
        base.FixedUpdate();
        attack();
    }

    void attack()
    {
        icon.color = Color.black;
        line.SetPosition(0, transform.position);
        line.SetPosition(1, transform.position);
        if (energy < Settings.inst.energy.toCellThresh) {
            return;
        }

        Collider2D[] near = Physics2D.OverlapCircleAll(transform.position, Settings.inst.weapon.attackRadius);
        foreach (var collider in near) {
            if (energy < Settings.inst.energy.toCellThresh) {
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
            icon.color = Color.white;
            line.SetPosition(1, obj.transform.position);
        }
    }
}

