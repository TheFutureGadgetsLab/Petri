using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class JeffJoint : MonoBehaviour
{
    SpringJoint2D spring;
    FixedJoint2D joint;
    LineRenderer line;
    new Rigidbody2D rigidbody;
    
    // Start is called before the first frame update
    void OnEnable()
    {
        spring = GetComponent<SpringJoint2D>();
        joint = GetComponent<FixedJoint2D>();
        line = GetComponent<LineRenderer>();
        rigidbody = GetComponent<Rigidbody2D>();
    }

    private void Start() {
        joint.connectedBody = transform.parent.GetComponent<Rigidbody2D>();
    }

    // Update is called once per frame
    void FixedUpdate()
    {    
        if (spring.connectedBody) {
            line.SetPosition(0, spring.connectedBody.position);
            line.SetPosition(1, transform.position);
        }
    }

    // Connect to another given jeff
    public void ConnectTo(JeffDoBeGoing otherJeff) {
        spring.connectedBody = otherJeff.GetComponent<Rigidbody2D>();
        spring.distance = 0.2f;
    }
}
