using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class JeffDoBeGoing : MonoBehaviour
{
    public Dictionary<GameObject, JeffJoint> joints = new Dictionary<GameObject, JeffJoint>();
    protected new Rigidbody2D rigidbody;
    protected GameObject jointPrefab;

    // Start is called before the first frame update
    protected void Start()
    {
        //var force = new Vector2(Random.Range(-500f, 500f), Random.Range(-500f, 500f));

        rigidbody = GetComponent<Rigidbody2D>();
        // rigidbody.AddForce(force);

        jointPrefab = Resources.Load<GameObject>("Joint");
    }

    // Update is called once per frame
    void FixedUpdate()
    {
    }

    // Called when colliding with another rigidbody
    void OnCollisionEnter2D(Collision2D col) {
        if (col.gameObject.GetComponent<JeffDoBeGoing>()
            && col.relativeVelocity.magnitude > 10.0f
            && !joints.ContainsKey(col.gameObject)
            && !col.gameObject.GetComponent<JeffDoBeGoing>().joints.ContainsKey(gameObject)) {
            var obj = GameObject.Instantiate(jointPrefab, Vector3.zero, Quaternion.identity);
            obj.transform.parent = transform;
            var jeffJoint = obj.GetComponent<JeffJoint>();
            jeffJoint.transform.localPosition = Vector3.zero;
            jeffJoint.ConnectTo(col.gameObject.GetComponent<JeffDoBeGoing>());
            joints.Add(col.gameObject, jeffJoint);
        }
    }
}
