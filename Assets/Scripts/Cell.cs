using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Cell : MonoBehaviour
{
    public Dictionary<GameObject, FixedJoint2D> joints = new Dictionary<GameObject, FixedJoint2D>();
    protected new Rigidbody2D rigidbody;

    public float energy = 0.0f;

    public static List<(GameObject prefab, float chance)> prefabs = null;

    public int organismID;

    protected void Awake() {
        organismID = Random.Range(int.MinValue, int.MaxValue);
        rigidbody = GetComponent<Rigidbody2D>();
    }

    protected void FixedUpdate()
    {
        foreach (var joint in joints) {
            Cell neighbor = joint.Key.GetComponent<Cell>();
            if (neighbor.organismID != organismID) {
                int maxID = Mathf.Max(neighbor.organismID, organismID);
                neighbor.organismID = maxID;
                organismID = maxID;
            }

            if (energy > Settings.inst.energy.toCellThresh
                && energy >= Settings.inst.cell.shareRate && neighbor.energy < energy) {
                neighbor.energy += Settings.inst.cell.shareRate;
                energy -= Settings.inst.cell.shareRate;
            }
        }

        if (energy < Settings.inst.cell.minEnergy) {
            destabilize();
        }
    }

    void destabilize()
    {
        foreach (var joint in joints) {
            var cell = joint.Key.GetComponent<Cell>();
            GameObject.Destroy(joint.Value);
            cell.joints.Remove(gameObject);
            cell.organismID = Random.Range(int.MinValue, int.MaxValue);
        }
        var newEnergy = GameObject.Instantiate(
            Energy.prefab,
            transform.position,
            Quaternion.identity
        );
        newEnergy.GetComponent<Energy>().energy = energy;

        joints.Clear();
        GameObject.Destroy(gameObject);
    }

    // Called when colliding with another rigidbody
    void OnCollisionEnter2D(Collision2D col)
    {
        handleCollision(col);
        handleEnergy(col);
    }

    void handleCollision(Collision2D col)
    {
        var otherCell = col.gameObject.GetComponent<Cell>();
        if (otherCell == null) {
            return;
        }

        //Bond formation
        if (joints.Count < Settings.inst.cell.maxBonds
            && col.relativeVelocity.magnitude > Settings.inst.cell.bondForce
            && otherCell.organismID != organismID
            && !joints.ContainsKey(otherCell.gameObject)
            && !otherCell.joints.ContainsKey(gameObject))
        {
            var joint = gameObject.AddComponent<FixedJoint2D>();
            otherCell.transform.position =
                transform.position + (otherCell.transform.position - transform.position).normalized * 1.15f;
            joint.connectedBody = otherCell.rigidbody;
            joint.enableCollision = true;
            joint.dampingRatio = 1f;
            joint.frequency = 0f;
            joints.Add(col.gameObject, joint);
            otherCell.joints.Add(gameObject, joint);

            otherCell.organismID = organismID;
        }
    }

    void handleEnergy(Collision2D col)
    {
        var energyObj = col.gameObject.GetComponent<Energy>();
        if (energyObj == null) {
            return;
        }

        energy += energyObj.energy;
        
        GameObject.Destroy(col.gameObject);
    }
}
