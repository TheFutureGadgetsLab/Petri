using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Propulsion : JeffDoBeGoing
{
    float force;

    new protected void Start() {
        base.Start();
        force = Random.Range(0.5f, 10.0f);
        transform.Rotate(Vector3.forward * Random.Range(-180, 180));
    }

    private void FixedUpdate() {
        if (rigidbody.velocity.magnitude < force) {
            rigidbody.AddRelativeForce(new Vector2(force - rigidbody.velocity.magnitude, 0.0f));
        }
    }
}
