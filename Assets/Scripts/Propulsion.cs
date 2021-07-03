using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Propulsion : Cell
{
    ConstantForce2D prop;
    SpriteRenderer icon;
    Vector2 force;
    float torque;

    new private void Awake() {
        base.Awake();
        icon = transform.Find("Icon").GetComponent<SpriteRenderer>();
    }

    protected void Start() {

        prop = GetComponent<ConstantForce2D>();
        force = Vector2.right * Settings.inst.propulsion.force;
        torque = Settings.inst.propulsion.torque.sample();
        transform.Rotate(Vector3.forward * 360f * Random.Range(0.0f, 1.0f));
    }

    new private void FixedUpdate() {
        if (energy > Settings.inst.propulsion.cost) {
            if (rigidbody.velocity.magnitude < Settings.inst.propulsion.speedLimit) {
                energy -= Settings.inst.propulsion.cost;
                prop.relativeForce = force;
                prop.torque = torque;
                icon.color = Color.white;
            } else {
                prop.relativeForce = Vector2.zero;
                prop.torque = 0.0f;
                icon.color = Color.grey;
                // We have more energy than we need, let's share it
                base.FixedUpdate();
            }
        } else {
            prop.relativeForce = Vector2.zero;
            prop.torque = 0.0f;
            icon.color = Color.black;
        }
        if (Mathf.Abs(rigidbody.angularVelocity) > Mathf.Abs(torque)) {
            prop.torque = 0.0f;
        }
    }
}

