using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Cell : MonoBehaviour
{
    public Dictionary<GameObject, Bond> joints = new Dictionary<GameObject, Bond>();
    protected new Rigidbody2D rigidbody;
    protected GameObject bondPrefab;

    public double food = 0.0;

    private CellParams config;

    protected void Awake() {
        config = GameObject.Find("Settings").GetComponent<Settings>().cellParams;
        rigidbody = GetComponent<Rigidbody2D>();
        bondPrefab = Resources.Load<GameObject>("Bond");
    }

    protected void FixedUpdate()
    {
        foreach (var joint in joints) {
            Cell neighbor = null;

            if (joint.Key.GetComponent<Cell>())
                neighbor = joint.Key.GetComponent<Cell>();

            if (food >= config.shareRate) {
                neighbor.food += config.shareRate;
                food -= config.shareRate;
            }
        }
    }

    // Called when colliding with another rigidbody
    void OnCollisionEnter2D(Collision2D col)
    {
        handleCollision(col);
        handleFood(col);
    }

    void handleCollision(Collision2D col)
    {
        var otherCell = col.gameObject.GetComponent<Cell>();
        if (otherCell == null) {
            return;
        }

        if (joints.Count < config.maxBonds
            && col.relativeVelocity.magnitude > config.bondForce
            && !joints.ContainsKey(col.gameObject)
            && !otherCell.joints.ContainsKey(gameObject))
        {
            var obj = GameObject.Instantiate(bondPrefab, Vector3.zero, Quaternion.identity);
            obj.transform.parent = transform;
            var cellJoint = obj.GetComponent<Bond>();
            cellJoint.transform.localPosition = Vector3.zero;
            cellJoint.ConnectTo(otherCell);
            joints.Add(col.gameObject, cellJoint);
            otherCell.joints.Add(gameObject, cellJoint);
        }
    }

    void handleFood(Collision2D col)
    {
        var foodObj = col.gameObject.GetComponent<Food>();
        if (foodObj == null) {
            return;
        }

        food += foodObj.food;
        
        GameObject.Destroy(col.gameObject);
    }
}
