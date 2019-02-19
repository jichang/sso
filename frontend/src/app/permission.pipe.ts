import { Pipe, PipeTransform } from "@angular/core";
import { ResourceType, ActionType } from "./permission-model.service";

@Pipe({
  name: "permission"
})
export class PermissionPipe implements PipeTransform {
  transform(value: any, args?: any): any {
    return null;
  }
}

@Pipe({
  name: "resourceType"
})
export class ResourceTypePipe implements PipeTransform {
  transform(value: any, args?: any): any {
    switch (value) {
      case ResourceType.None:
        return "None";
      case ResourceType.Application:
        return "Application";
      case ResourceType.Group:
        return "Group";
      case ResourceType.GroupUser:
        return "GroupUser";
      case ResourceType.Permission:
        return "Permission";
      case ResourceType.RoleUser:
        return "RoleUser";
      case ResourceType.RolePermisson:
        return "RolePermission";
      case ResourceType.User:
        return "User";
    }
  }
}

@Pipe({
  name: "actionType"
})
export class ActionTypePipe implements PipeTransform {
  transform(value: any, args?: any): any {
    switch (value) {
      case ActionType.NONE:
        return "None";
      case ActionType.CREATE:
        return "Create";
      case ActionType.SELECT:
        return "Select";
      case ActionType.UPDATE:
        return "Update";
      case ActionType.DELETE:
        return "Delete";
    }
  }
}
