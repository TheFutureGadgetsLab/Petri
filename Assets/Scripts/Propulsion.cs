using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Propulsion : JeffDoBeGoing
{
    public Vector2 minMaxForce = new Vector2(0.0f, 10.0f);
    public Vector2 minMaxTorque = new Vector2(-7.0f, 7.0f);

    new protected void Start() {
        base.Start();
        Debug.Log("Setting force");

        // Set a random rotation
        transform.Rotate(Vector3.forward * Random.Range(-180, 180));

        var constantForce = GetComponent<ConstantForce2D>();
        constantForce.relativeForce = new Vector2(Random.Range(minMaxForce.x, minMaxForce.y), Random.Range(minMaxForce.x, minMaxForce.y));

        constantForce.torque = Random.Range(minMaxTorque.x, minMaxTorque.y);
    }
}
